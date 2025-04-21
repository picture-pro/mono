use base_components::{
  utils::{inputs::touched_input_bindings, navigation::navigate_to},
  FloatingBoxSection,
};
use leptos::prelude::*;
use lsc::{button::*, field::*};
use models::{EmailAddress, HumanName, HumanNameError, UserRecordId};

#[derive(Clone, PartialEq)]
enum SignupFormState {
  Untouched,
  ValidationFailed,
  ReadyToSubmit,
  Pending,
  Succeeded,
}

#[component]
fn SubmitButton(#[prop(into)] state: Signal<SignupFormState>) -> impl IntoView {
  use SignupFormState::*;

  let disabled = Signal::derive(move || {
    matches!(state(), ValidationFailed | Pending | Succeeded)
  });

  let icon_fragment = move || match state() {
    Pending => leptos::either::Either::Left(view! {
      <lsc::icons::ArrowRightIcon {..} class="size-5 animate-spin" />
    }),
    _ => leptos::either::Either::Right(view! {
      <lsc::icons::ArrowRightIcon {..} class="size-5" />
    }),
  };

  view! {
    <Button
      size=ButtonSize::Large disabled=disabled
    >
      "Sign up"
      { icon_fragment }
    </Button>
  }
}

#[component]
fn FieldErrorText(text: &'static str) -> impl IntoView {
  view! {
    <p class="text-sm text-dangera-11 dark:text-dangerdarka-11">
      { text }
    </p>
  }
}

// #[component]
// fn FieldWarningText(text: &'static str) -> impl IntoView {
//   view! {
//     <p class="text-sm text-warninga-11 dark:text-warningdarka-11">
//       { text }
//     </p>
//   }
// }

#[component]
fn FormField(
  field_id: &'static str,
  placeholder: &'static str,
  #[prop(optional)] field_type: Option<&'static str>,
  field_label: &'static str,
  #[prop(into)] hint: Signal<Option<FieldHint>>,
  signal: RwSignal<Option<String>>,
  children: Children,
) -> impl IntoView {
  let (read, write) = touched_input_bindings(signal);
  view! {
    <div class="flex flex-col gap-1">
      <label class="" for=field_id>{ field_label }</label>
      <Field size={FieldSize::Large} hint={hint} {..}
        placeholder=placeholder id=field_id type=field_type
        on:input=write prop:value=read
      />
      { children() }
    </div>
  }
}

#[island]
pub fn SignupPage() -> impl IntoView {
  // the actual input values
  // we store them as Option<String> so that we don't run validation on
  //   untouched inputs
  let name = RwSignal::new(None::<String>);
  let email = RwSignal::new(None::<String>);
  let confirm_email = RwSignal::new(None::<String>);

  // validated versions of the input values
  let validated_name =
    Memo::new(move |_| name.read().as_ref().map(HumanName::try_new));
  let validated_email =
    Memo::new(move |_| email.read().as_ref().map(EmailAddress::try_new));
  let validated_confirm_email = Memo::new(move |_| {
    confirm_email
      .read()
      .as_ref()
      .and_then(|c| email.read().as_ref().map(|e| !c.eq(e)))
  });

  let action = Action::new(move |_: &()| {
    signup(
      name.get().unwrap_or_default(),
      email.get().unwrap_or_default(),
      confirm_email.get().unwrap_or_default(),
    )
  });
  let action_value = action.value();
  let action_pending = action.pending();
  let action_succeeded = move || matches!(action_value.get(), Some(Ok(_)));
  let action_value_view = move || {
    action_value.get().map(|v| match v {
      Ok(id) => leptos::either::Either::Left(view! {
        <p class="text-success-11 dark:text-successdark-11">"User created with id: " {id.to_string()}</p>
      }),
      Err(e) => leptos::either::Either::Right(view! {
        <p class="text-danger-11 dark:text-dangerdark-11">{ e.to_string() }</p>
      }),
    })
  };

  Effect::new(move |_| {
    if action_succeeded() {
      navigate_to("/profile");
    }
  });

  // error signals
  let name_error = Memo::new(move |_| {
    validated_name().and_then(|r| {
      r.err().map(|e| match e {
        HumanNameError::NotEmptyViolated => "Name cannot be empty",
        HumanNameError::LenCharMaxViolated => "Name is too long",
      })
    })
  });
  // let email_error = Memo::new(move |_| {
  //   validated_email().and_then(|r| {
  //     match r.map(|e| models::validate_compliant_email_address(e.as_ref())) {
  //       Ok(true) => None,
  //       Ok(false) => Some(Either::Left(
  //         "Though technically valid, this email address is probably not \
  //          correct",
  //       )),
  //       Err(models::EmailAddressError::PredicateViolated) => {
  //         Some(Either::Right("Invalid email address"))
  //       }
  //       Err(models::EmailAddressError::LenCharMaxViolated) => {
  //         Some(Either::Right("Email is too long"))
  //       }
  //     }
  //   })
  // });
  let email_error = Memo::new(move |_| match validated_email() {
    Some(Err(models::EmailAddressError::PredicateViolated)) => {
      Some("Invalid email address")
    }
    Some(Err(models::EmailAddressError::LenCharMaxViolated)) => {
      Some("Email is too long")
    }
    Some(Ok(_)) | None => None,
  });
  let confirm_email_error = Memo::new(move |_| {
    validated_confirm_email().and_then(|r| r.then_some("Emails do not match"))
  });

  let state = Memo::new(move |_| {
    if name().is_none() && email().is_none() && confirm_email().is_none() {
      return SignupFormState::Untouched;
    }
    if name_error().is_some()
      || email_error().is_some()
      || confirm_email_error().is_some()
    {
      return SignupFormState::ValidationFailed;
    }
    if action_pending() {
      return SignupFormState::Pending;
    }
    if action_succeeded() {
      return SignupFormState::Succeeded;
    }
    SignupFormState::ReadyToSubmit
  });

  // views for input errors
  let name_error_view =
    move || name_error().map(|t| view! { <FieldErrorText text=t /> });
  let email_error_view = move || {
    email_error().map(|e| {
      // e.either(
      //   |t| view! { <FieldWarningText text=t /> }.into_any(),
      //   |t| view! { <FieldErrorText text=t /> }.into_any(),
      // )
      view! { <FieldErrorText text=e /> }.into_any()
    })
  };
  let confirm_email_error_view =
    move || confirm_email_error().map(|t| view! { <FieldErrorText text=t /> });

  // hint values for the input fields
  let name_input_hint =
    Signal::derive(move || name_error().map(|_| FieldHint::Error));
  let email_input_hint = Signal::derive(move || {
    email_error().map(|_| {
      // match e {
      //   Either::Left(_) => FieldHint::Warning,
      //   Either::Right(_) => FieldHint::Error,
      // }
      FieldHint::Error
    })
  });
  let confirm_email_input_hint =
    Signal::derive(move || confirm_email_error().map(|_| FieldHint::Error));

  view! {
    <FloatingBoxSection>
      <p class="text-3xl font-serif font-semibold tracking-tight">
        "Create your account"
      </p>
      <p class="text-base-dim max-w-prose">
        "Spend less time distributing your photos and more time capturing memories. Sign up for \
        PicturePro today."
      </p>

      <form class="mt-2 mb-4 flex flex-col gap-4">
        <FormField
          field_id="name" placeholder="Enter your full name" hint=name_input_hint
          field_label="Full Name" signal=name
        >
          { name_error_view }
        </FormField>

        <FormField
          field_id="email" placeholder="Enter your email" hint=email_input_hint
          field_label="Email" signal=email field_type="email"
        >
          { email_error_view }
        </FormField>

        <FormField
          field_id="confirm_email" placeholder="Enter your email again"
          hint=confirm_email_input_hint field_label="Confirm Email" signal=confirm_email
          field_type="email"
        >
          { confirm_email_error_view }
        </FormField>
      </form>

      <div class="flex flex-row">
        <div class="flex-1" />
        // <Button size={ButtonSize::Large} {..} on:click={move |_| {action.dispatch(());}}>
        //   "Sign up"
        //   <lsc::icons::ArrowRightIcon {..} class="size-5" />
        // </Button>
        <SubmitButton state={state} {..} on:click={move |_| {action.dispatch(());}} />
      </div>

      { move || action_value_view().map(|v| view! {
        <div class="self-center mt-4">{ v }</div>
      })}
    </FloatingBoxSection>
  }
}

#[server(name = SignupActionParams)]
#[tracing::instrument]
async fn signup(
  name: String,
  email: String,
  confirm_email: String,
) -> Result<UserRecordId, ServerFnError> {
  use auth_domain::{AuthDomainService, AuthSession};
  use models::{UserAuthCredentials, UserCreateRequest};

  let auth_service = use_context::<AuthDomainService>().ok_or_else(|| {
    tracing::error!("auth service not found");
    ServerFnError::new("Internal error")
  })?;
  let mut auth_session =
    leptos_axum::extract::<AuthSession>().await.map_err(|_| {
      tracing::error!("auth session not found");
      ServerFnError::new("Internal error")
    })?;

  let name = HumanName::try_new(name).map_err(|e| match e {
    HumanNameError::NotEmptyViolated => ServerFnError::new("Name is empty"),
    HumanNameError::LenCharMaxViolated => {
      ServerFnError::new("Name is too long")
    }
  })?;
  let email = EmailAddress::try_new(email)
    .map_err(|_| ServerFnError::new("Email address is invalid"))?;
  // if !models::validate_reasonable_email_address(email.as_ref()) {
  //   return Err(ServerFnError::new("Email is unreasonable"));
  // }
  let confirm_email = EmailAddress::try_new(confirm_email)
    .map_err(|_| ServerFnError::new("Emails do not match"))?;
  if email != confirm_email {
    return Err(ServerFnError::new("Emails do not match"));
  }

  let creds = UserAuthCredentials::EmailEntryOnly(email.clone());
  let create_request = UserCreateRequest {
    name,
    email,
    auth: creds.clone(),
  };

  let user =
    auth_service
      .user_signup(create_request)
      .await
      .map_err(|e| match e {
        auth_domain::CreateUserError::EmailAlreadyUsed(email) => {
          tracing::warn!(
            "failed to create user: email already in use: \"{email}\""
          );
          ServerFnError::new("Email is already in use")
        }
        auth_domain::CreateUserError::CreateError(error) => {
          tracing::error!("failed to create user: {error}");
          ServerFnError::new("Internal error")
        }
        auth_domain::CreateUserError::FetchByIndexError(error) => {
          tracing::error!("failed to create user: {error}");
          ServerFnError::new("Internal error")
        }
      })?;

  let public_user = models::PublicUser::from(user.clone());
  auth_session.login(&public_user).await.map_err(|e| {
    tracing::error!("failed to log in user: {e}");
    ServerFnError::new("Internal error")
  })?;

  Ok(user.id)
}
