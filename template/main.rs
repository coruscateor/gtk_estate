mod applicaion_state;

mod window_state;

use gtk_estate::{adw::{prelude::*, Application}, StateContainers};

use crate::applicaion_state::ApplicattionState;

mod window_contents_state;

mod actors;

fn main()
{

    let app = Application::builder().application_id("org.escape_it_gui").build();

    //This instance of the State containers is needed for its global access to work

    //The static mut vaiable contains a weak reference to the below reference counted StateContainers instance

    let _sc = StateContainers::new();
    
    ApplicattionState::new(&app);

    app.run();

}
