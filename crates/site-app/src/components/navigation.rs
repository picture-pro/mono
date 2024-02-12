use leptos::*;

pub struct ClientNav<
  S: Fn() -> bool + Copy + 'static,
  P: Fn() -> String + Copy + 'static,
>(S, ClientNavInner<P>);

impl<S: Fn() -> bool + Copy + 'static, P: Fn() -> String + Copy + 'static>
  ClientNav<S, P>
{
  pub fn new(is_active: S, path: P) -> Self {
    Self(is_active, ClientNavInner::new(path))
  }
}

impl<S: Fn() -> bool + Copy + 'static, P: Fn() -> String + Copy + 'static>
  IntoView for ClientNav<S, P>
{
  fn into_view(self) -> View {
    {
      move || match self.0() {
        true => self.1.into_view(),
        false => view! {}.into_view(),
      }
    }
    .into_view()
  }
}

#[derive(Copy, Clone)]
pub struct ClientNavInner<P: Fn() -> String + Copy + 'static>(P);

impl<P: Fn() -> String + Copy + 'static> ClientNavInner<P> {
  pub fn new(path: P) -> Self { Self(path) }
}

impl<P: Fn() -> String + Copy + 'static> IntoView for ClientNavInner<P> {
  fn into_view(self) -> View {
    {
      move || {
        view! {
          <script>
            {format!("document.location.href = '{}';", self.0())}
          </script>
        }
      }
    }
    .into_view()
  }
}
