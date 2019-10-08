pub trait TemplateInstaller {
    fn new() -> Self where Self: Sized;
    fn install(&self);
}