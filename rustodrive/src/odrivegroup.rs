use std::collections::BTreeMap;

use crate::{
    axis::{Axis, AxisID},
    canframe::{ticket, CANRequest, CANResponse, ODriveCANFrame},
    response::{ErrorResponse, ODriveError, ODriveResponse, ResponseType, Success},
    state::{ODriveCommand::Write, WriteComm::*},
    threads::ReadWriteCANThread,
};

/// `ODriveGroup` is an interface for communicating with the odrive,
/// without having to worry about creating the boilerplate `ODriveCANFrame`
/// over and over again.
///
/// To initialize, we pass in a slice of all the axis IDs. This is not the
/// same as the normal axis ID for a single ODrive. For each odrive connected
/// on the CAN interface, you must specify the can_node_id
/// ```python
/// # ODrive Python configuration
/// odrv0.axis0.config.can.node_id = 0
/// odrv0.axis1.config.can.node_id = 1
/// odrv1.axis0.config.can.node_id = 2
/// odrv1.axis1.config.can.node_id = 3
/// ```
/// ### Talking to the odrive
/// To interact with the ODrive through CAN, the two most common methods are
/// [`ODriveGroup::axis()`] and [`ODriveGroup::all_axes()`] as seen in the example.
/// You may pass CANFrames directly or use the preconfigured ones in the [`Axis`] struct
///
/// # Example
/// ```
/// // rust code
/// use std::error::Error;
/// use std::time::Duration;
/// use rustodrive::{
///     canproxy::CANProxy,
///     odrivegroup::{ODriveGroup},
///     state::AxisState::*, threads::ReadWriteCANThread
/// };
///
/// fn main() {
///     let mut can_proxy = CANProxy::new("can0");
///     can_proxy.register_rw("thread 1", odrive_main);
///
///     let stop_all = can_proxy.begin();
///     std::thread::sleep(Duration::from_secs(1));
///
///     stop_all().unwrap();
///     println!("all done!");
/// }
///
/// // Entrypoint for odrive control
/// fn odrive_main(can_rw: ReadWriteCANThread) {
///     let odrives = ODriveGroup::new(can_rw, &[1, 2, 3, 4]);

///     println!("Starting calibration sequence");
///     odrives.all_axes(|ax| ax.set_state(FullCalibrationSequence));
///     println!("Motors fully calibrated!")
/// }
/// ```

pub struct ODriveGroup<'a> {
    can: ReadWriteCANThread,
    axes: BTreeMap<&'a AxisID, Axis<'a>>,
}

impl<'a> ODriveGroup<'a> {
    pub fn new(can: ReadWriteCANThread, axis_ids: &'static [AxisID]) -> Self {
        ODriveGroup {
            axes: axis_ids.iter().map(|id| (id, Axis::new(id))).collect(),
            can,
        }
    }

    /// This method sends the request specified by the closure to all the axes simultaneously
    /// and blocks until they all come back. Conversely, `.axis()` sends a request to only 1
    /// axis and blocks until it receives a response.
    ///
    /// If you so choose, you can create the requests by hand, but, `Axis` exposes an
    /// interface that contains premade methods that generate boilerplate requests for you.
    ///
    /// # Arguments
    /// * `f` - a closure that takes an [`Axis`] as a parameter and returns a [`CANRequest`]
    ///
    /// ### Example
    /// This will start the calibration sequence for all motors simultaneously.
    ///
    /// ```
    /// use std::time::Duration;
    /// use rustodrive::odrivegroup::ODriveGroup;
    /// use rustodrive::canproxy::CANProxy;
    /// use rustodrive::state::AxisState::FullCalibrationSequence;
    ///
    /// let mut can_proxy = CANProxy::new("can0");
    /// can_proxy.register_rw("thread 1", |can_rw| {
    ///     let odrives = ODriveGroup::new(can_rw, &[1, 2, 3, 4]);
    ///     odrives.all_axes(|ax| ax.set_state(FullCalibrationSequence));
    /// });
    ///
    /// let stop = can_proxy.begin();
    /// std::thread::sleep(Duration::from_secs(1));
    /// stop();
    /// ```
    pub fn all_axes<T: TryFrom<CANResponse, Error = ODriveError>, F>(
        &self,
        mut f: F,
    ) -> Vec<Result<Success<T>, ErrorResponse>>
    where
        F: FnMut(&Axis) -> CANRequest,
    {
        let requests = self.axes.values().map(|ax| f(ax)).collect();
        let responses = self.can.request_many(requests);

        let mut final_responses = vec![];
        for res in responses {
            let res = Self::convert_response(res);
            final_responses.push(res);
        }

        final_responses
    }

    /// This method sends the request specified by the closure to the axis specified.
    /// Conversely, `.all_axes()` sends the request to all axes simulatenously and
    /// blocks until it receives a response.
    ///
    /// If you so choose, you can create the requests by hand, but, `Axis` exposes an
    /// interface that contains premade methods that generate boilerplate requests for you.
    ///
    /// # Arguments
    /// * `f` - a closure that takes an [`Axis`] as a parameter and returns a [`CANRequest`]
    ///
    /// ### Example
    /// This will start the calibration sequence for axis 1
    ///
    /// ```
    /// use std::time::Duration;
    /// use rustodrive::odrivegroup::ODriveGroup;
    /// use rustodrive::canproxy::CANProxy;
    /// use rustodrive::state::AxisState::FullCalibrationSequence;
    ///
    /// let mut can_proxy = CANProxy::new("can0");
    /// can_proxy.register_rw("thread 1", |can_rw| {
    ///     let odrives = ODriveGroup::new(can_rw, &[1, 2, 3, 4]);
    ///     odrives.axis(&1, |ax| ax.set_state(FullCalibrationSequence));
    /// });
    ///
    /// let stop = can_proxy.begin();
    /// std::thread::sleep(Duration::from_secs(1));
    /// stop();
    /// ```
    pub fn axis<T: TryFrom<ODriveCANFrame, Error = ODriveError>, F: FnOnce(&Axis) -> CANRequest>(
        &self,
        axis_id: &AxisID,
        f: F,
    ) -> Result<Success<T>, ErrorResponse>
    {
        Self::convert_response(self.can.request(f(self.get_axis(axis_id))))
    }

    fn convert_response<T: TryFrom<CANResponse, Error = ODriveError>>(
        response: ODriveResponse,
    ) -> Result<Success<T>, ErrorResponse> {
        // For each received response, either add the error to the responses or
        // convert the response to the appropriate type
        let resp_type = match response {
            Ok(resp) => resp,
            Err(err) => return Err(err),
        };

        // We convert a response to the generic type if it has a body. Otherwise, we convert
        // the request made to the response so that it can be converted to the () type
        let can_to_convert = match resp_type {
            ResponseType::Body {
                request: _,
                response,
            } => response,
            ResponseType::Bodyless { req } => req,
        };

        // We use the request to check if the command sent is a Read request. If it is, it panics. Otherwise it returns ()
        // If there is bad data that is a recoverable error, return ODriveError
        return match can_to_convert.try_into() {
            Ok(data) => Ok(Success {
                axis: can_to_convert.axis as usize,
                sent_request: resp_type.request(),
                data,
            }),
            Err(e) => Err(ErrorResponse {
                request: resp_type.request(),
                err: e,
            }),
        };
    }

    fn get_axis(&self, id: &AxisID) -> &Axis {
        match self.axes.get(id) {
            Some(axis) => axis,
            None => panic!("Cannot retrieve axis {} that doesn't exist!", id),
        }
    }

    fn first_axis_id(&self) -> usize {
        **self.axes.iter().next().unwrap().0
    }

    pub fn reboot(&self) -> () {
        ticket(self.first_axis_id(), Write(RebootODrive), [0; 8]);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::canframe::CANRequest;
    use crate::canproxy::CANProxy;
    use crate::casts::Temperature;
    use crate::response::Success;
    use crate::state::ReadComm;
    use crate::state::{AxisState::{*, self}, ODriveCommand, WriteComm};
    use crate::tests::wait_for_msgs;
    use crate::utils::ResultAll;

    use super::ODriveGroup;

    #[test]
    fn test_axes() {
        let mut proxy = CANProxy::new("fakecan");

        let (send, rcv) = channel();

        let expected_request = CANRequest {
            axis: 1,
            cmd: ODriveCommand::Read(ReadComm::GetTemperature),
            data: [0, 0, 0, 0, 0, 0, 0, 0],
        };

        proxy.register_rw("thread 1", move |can_rw| {
            let odrives = ODriveGroup::new(can_rw, &[0, 1, 2, 3, 4, 5]);

            let responses: Success<Temperature> = odrives.axis(&1, |ax| ax.get_temperatures()).unwrap();
            send.send(responses).unwrap();
        });
        let stop_all = proxy.begin();

        // test the that all the results are returned in the order they were sent
        let response = wait_for_msgs(rcv);
        stop_all().unwrap();

        assert_eq!(response.sent_request, expected_request);
    }

    #[test]
    fn test_all_axes() {
        let mut proxy = CANProxy::new("fakecan");

        let (send, rcv) = channel();

        let mut expected_requests = Vec::new();
        for i in 0..6 {
            expected_requests.push(CANRequest {
                axis: i,
                cmd: ODriveCommand::Write(WriteComm::SetAxisRequestedState),
                data: [FullCalibrationSequence as u8, 0, 0, 0, 0, 0, 0, 0],
            })
        }

        proxy.register_rw("thread 1", move |can_rw| {
            let odrives = ODriveGroup::new(can_rw, &[0, 1, 2, 3, 4, 5]);

            let responses: Vec<Success<()>> =
                odrives.all_axes(|ax| ax.set_state(AxisState::FullCalibrationSequence)).unwrap_all();
            send.send(responses).unwrap();
        });
        let stop_all = proxy.begin();

        // test the that all the results are returned in the order they were sent
        let response = wait_for_msgs(rcv);
        stop_all().unwrap();

        for (request, response) in expected_requests.into_iter().zip(response) {
            assert_eq!(response.sent_request, request);
        }
    }
}
