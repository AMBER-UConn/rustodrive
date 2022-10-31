This module is the frontend for the rustodrive module. It is still in development and operates on mock data for testing purposes. 

The frontend is broken up into several components. Shared modules and views. Views define the main windows used in the application and the associated state of each window. For example, the `control_panel.rs` file defines all state and components to render for the main control panel table and no other state. These files are isolated from the other views. All the views are composed in the main function and are rendered based on logic determined in `ui_main()`. 

`shared/` defines reusable components and functionality that is accessed in the views. 
- `state.rs` breaks up state into application and backend. Application state is all state that can change visually and does not affect hardware state. The backend is responsible for taking in application state and updating the hardware when is seen fit.

- `widgets.rs` defines independent user interface widgets such as checkboxes, plotters, and dropdown menus

- `components.rs` builds off of `widgets.rs` to create ODrive specific re-useable widgets such as dropdowns specifically for selecting drive state.