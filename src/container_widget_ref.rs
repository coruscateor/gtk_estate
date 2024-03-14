use std::any::Any;

use gtk::glib::{object::ObjectExt, types::StaticType};

use crate::gtk4 as gtk;


pub enum ContainerWidgetRef<'a>
{

    //GTK

    //GtkApplication(gtk::Application),
    GtkApplicationWindow(&'a gtk::ApplicationWindow),
    GtkAspectFrame(&'a gtk::AspectFrame),
    GtkBox(&'a gtk::Box),
    GtkCenterBox(&'a gtk::CenterBox),
    GtkExpander(&'a gtk::Expander),
    GtkFixed(&'a gtk::Fixed),
    GtkFlowBox(&'a gtk::FlowBox),
    GtkFlowBoxChild(&'a gtk::FlowBoxChild),
    GtkGrid(&'a gtk::Grid),
    //GtkGridLayout(&'a gtk::GridLayout),
    //GtkGridLayoutChild(&'a gtk::GridLayoutChild),
    GtkGridView(&'a gtk::GridView),
    GtkNotebook(&'a gtk::Notebook),
    GtkOverlay(&'a gtk::Overlay),
    GtkRevealer(&'a gtk::Revealer),
    GtkScrolledWindow(&'a gtk::ScrolledWindow),
    GtkSearchBar(&'a gtk::SearchBar),
    GtkStack(&'a gtk::Stack),
    GtkViewport(&'a gtk::Viewport),
    GtkWindow(&'a gtk::Window),

    //WidgetPaintable? - https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.WidgetPaintable.html

    //ADW

}

impl<'a> ContainerWidgetRef<'a>
{

    pub fn new<T>(container_widget: &T) -> Result<ContainerWidgetRef, &'static str>
        where T: ObjectExt //+ Clone
    {

        let cwt = container_widget.type_();

        let any_container_widget: &dyn Any = container_widget;

        let unexpected_error_message = "Error: Unexpected cast error.";

        //GTK

        if gtk::ApplicationWindow::static_type() == cwt
        {

            let wref = any_container_widget.downcast_ref::<gtk::ApplicationWindow>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkApplicationWindow(wref));

        }
        else if gtk::AspectFrame::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::AspectFrame>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkAspectFrame(wref));

        }
        else if gtk::Box::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Box>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkBox(wref));

        }
        else if gtk::CenterBox::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::CenterBox>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkCenterBox(wref));

        }
        else if gtk::Expander::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Expander>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkExpander(wref));

        }
        else if gtk::Fixed::static_type() == cwt
        {

            let wref = any_container_widget.downcast_ref::<gtk::Fixed>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkFixed(wref));

        }
        else if gtk::FlowBox::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::FlowBox>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkFlowBox(wref));

        }
        else if gtk::FlowBoxChild::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::FlowBoxChild>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkFlowBoxChild(wref));

        }
        else if gtk::Grid::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Grid>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkGrid(wref));

        }
        else if gtk::Grid::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Grid>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkGrid(wref));

        }
        else if gtk::GridView::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::GridView>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkGridView(wref));

        }
        else if gtk::Notebook::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Notebook>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkNotebook(wref));

        }
        else if gtk::Overlay::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Overlay>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkOverlay(wref));

        }
        else if gtk::Revealer::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Revealer>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkRevealer(wref));

        }
        else if gtk::ScrolledWindow::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::ScrolledWindow>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkScrolledWindow(wref));

        }
        else if gtk::SearchBar::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::SearchBar>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkSearchBar(wref));

        }
        else if gtk::Stack::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Stack>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkStack(wref));

        }
        else if gtk::Viewport::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Viewport>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkViewport(wref));

        }
        else if gtk::Window::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Window>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkWindow(wref));

        }

        //ADW
        
        Err("Error: Widget of invalid type provided.")

        /*
        match cwt.name()
        {

            gtk::ApplicationWindow::static_type().name() => {

                

            }
            
        }
        */

    }

}