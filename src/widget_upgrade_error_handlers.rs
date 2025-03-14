use crate::{WidgetUpgradeError, WidgetUpgradeResult};

///
/// Calls error_fn if the provided result is an Err.
/// 
/// Returns true if error_fn was called.
/// 
pub fn on_widget_upgrade_error<F>(result: WidgetUpgradeResult, error_fn: F) -> bool
    where F: FnOnce(WidgetUpgradeError)
{

    if let Err(err) = result
    {

        error_fn(err);

        return true;

    }

    false

}

///
/// Calls error_fn if the provided result is an Err.
/// 
/// error_fn takes the provided parameter P in addition to the WidgetUpgradeError.
/// 
/// Returns true if error_fn was called.
/// 
pub fn on_widget_upgrade_error_with_param<F, P>(result: WidgetUpgradeResult, error_fn: F, param: P) -> bool
    where F: FnOnce(WidgetUpgradeError, P)
{

    if let Err(err) = result
    {

        error_fn(err, param);

        return true;

    }

    false

}

///
/// If the provided result is an Err it is displayed in the standard-out.
///  
pub fn widget_upgrade_error_display_println(result: WidgetUpgradeResult) -> bool
{

    on_widget_upgrade_error(result, |err|
    {

        println!("Widget upgrade error: {}\n\n", err);

    })

}

///
/// If the provided result is an Err it is debug formatted and displayed in the standard-out.
/// 
pub fn widget_upgrade_error_debug_println(result: WidgetUpgradeResult) -> bool
{

    on_widget_upgrade_error(result, |err|
    {

        println!("Widget upgrade error: {:?}\n\n", err);

    })

}

///
/// If the provided result is an Err, the function panics.
///  
pub fn widget_upgrade_error_display_panic(result: WidgetUpgradeResult) -> bool
{

    on_widget_upgrade_error(result, |err|
    {

        panic!("Widget upgrade error: {}", err);

    })

}

///
/// If the provided result is an Err, the function panics with a debug formatted message.
///  
pub fn widget_upgrade_error_debug_panic(result: WidgetUpgradeResult) -> bool
{

    on_widget_upgrade_error(result, |err|
    {

        panic!("Widget upgrade error: {:?}", err);

    })

}


