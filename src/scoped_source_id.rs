use gtk4 as gtk;

use gtk::glib::SourceId;

///
/// Makes it easier to handle SourceIds.
/// 
pub struct ScopedSourceId
{

    opt_source_id: Option<SourceId>

}

impl ScopedSourceId
{

    pub fn new(source_id: SourceId) -> Self
    {

        Self
        {

            opt_source_id: Some(source_id)

        }

    }

    pub fn empty() -> Self
    {

        Self
        {

            opt_source_id: None

        }

    }

    pub fn has_source_id(&self) -> bool
    {

        self.opt_source_id.is_some()

    }

    pub fn is_empty(&self) -> bool
    {

        self.opt_source_id.is_none()

    }

    pub fn try_as_raw(&self) -> Option<u32>
    {

        if let Some(source) = &self.opt_source_id
        {

            Some(source.as_raw())

        }
        else
        {

            None    

        }

    }

    pub fn try_remove(&mut self) -> bool
    {

        if self.opt_source_id.is_some()
        {

            let source = self.opt_source_id.take().unwrap();

            source.remove();

            true

        }
        else
        {

            false
            
        }

        /*
        if let Some(source) = &mut self.opt_source_id
        {

            /*
            `*source` moved due to this method callrustcE0507
            scoped_source_id.rs(45, 13): original diagnostic
            cannot move out of `*source` which is behind a mutable reference
            move occurs because `*source` has type `SourceId`, which does not
            */
            
        }
        else
        {

            false
            
        }
            source.remove();

            self.opt_source_id = None;

            true

        }
        else
        {

            false
            
        }
        */

    }
    
}

impl Drop for ScopedSourceId
{

    fn drop(&mut self)
    {

        self.try_remove();
       
    }
    
}

