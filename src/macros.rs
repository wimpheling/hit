#[macro_export]
macro_rules! modele {
    (
        $model_name:literal,
        $model_label:literal
         =>
            $(
                $field_name:literal: $field:ident $({
                    $($key: ident: $value: expr ),*$(,)?
                })?
            ),*
            $(,interfaces: $($interfaces: literal),*)?
            $(,)?
    ) => {
        {
            let mut mdl: indexed_model::Model = indexed_model::Model::new(
                String::from($model_name),
                String::from($model_label),
            );
            $($(mdl.fields.insert(
                String::from($field_name),
                std::rc::Rc::new(std::cell::RefCell::new($field {
                    name: String::from($field_name),
                    $($key: $value,)*
                    ..Default::default()
                }))
            );)*)?
            $($(mdl.interfaces.push(String::from($interfaces));),*)?
            std::rc::Rc::new(mdl)
        }
    }
}
