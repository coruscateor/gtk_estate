use std::rc::Rc;

use crate::StateContainers;

use corlib::impl_get_ref;

pub struct ClearStateContainersOnDrop
{

    state_containers: Rc<StateContainers>

}

impl ClearStateContainersOnDrop
{

    pub fn new(state_containers: Rc<StateContainers>) -> Self
    {

        Self
        {

            state_containers

        }

    }

    #[cfg(feature = "thread_local_state")]
    pub fn get() -> Self
    {

        Self::new(StateContainers::get())

    }

    impl_get_ref!(state_containers, Rc<StateContainers>);

}

impl Drop for ClearStateContainersOnDrop
{

    fn drop(&mut self)
    {

        self.state_containers.clear();
        
    }

}
