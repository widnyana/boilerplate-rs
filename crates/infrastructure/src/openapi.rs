use aide::transform::TransformPathItem;

pub fn openapi_tag<T: AsRef<str>>(tag: T) -> impl Fn(TransformPathItem<'_>) -> TransformPathItem<'_> {
	move |op| op.tag(tag.as_ref())
}
