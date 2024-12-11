use either::Either;
use leptos::{ev::Event, prelude::*};
use lsc::{button::*, field::*};
use models::{EmailAddress, HumanName, HumanNameError, UserRecordId};

use crate::components::FloatingBoxSection;

fn touched_input_bindings(
  s: RwSignal<Option<String>>,
) -> (impl Fn() -> String, impl Fn(Event)) {
  (
    move || s.get().unwrap_or_default(),
    move |e| {
      s.set(Some(event_target_value(&e)));
    },
  )
}

#[island]
pub fn SignupPage() -> impl IntoView {
  // the actual input values
  // we store them as Option<String> so that we don't run validation on
  //   untouched inputs
  let name = RwSignal::new(None::<String>);
  let email = RwSignal::new(None::<String>);
  let confirm_email = RwSignal::new(None::<String>);

  // bindings for the input values
  let (read_name_callback, write_name_callback) = touched_input_bindings(name);
  let (read_email_callback, write_email_callback) =
    touched_input_bindings(email);
  let (read_confirm_email_callback, write_confirm_email_callback) =
    touched_input_bindings(confirm_email);

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
  let action_value_view = move || {
    action_value().map(|v| match v {
      Ok(id) => leptos::either::Either::Left(view! {
        <p class="text-success-11 dark:text-successdark-11">"User created with id: " {id.to_string()}</p>
      }),
      Err(e) => leptos::either::Either::Right(view! {
        <p class="text-danger-11 dark:text-dangerdark-11">{ e.to_string() }</p>
      }),
    })
  };

  Effect::new(move |_| {
    if matches!(action_value(), Some(Ok(_))) {
      crate::utils::navigation::navigate_to("/");
    }
  });

  // error messages
  let error_message_class = "text-sm text-dangera-11 dark:text-dangerdarka-11";
  let warning_message_class =
    "text-sm text-warninga-11 dark:text-warningdarka-11";
  let error_view = move |e: &'static str| {
    view! { <p class=error_message_class>{ e }</p> }
  };
  let warning_view = move |e: &'static str| {
    view! { <p class=warning_message_class>{ e }</p> }
  };

  // error signals
  let name_error = Memo::new(move |_| {
    validated_name().and_then(|r| {
      r.err().map(|e| match e {
        HumanNameError::NotEmptyViolated => "Name cannot be empty",
        HumanNameError::LenCharMaxViolated => "Name is too long",
      })
    })
  });
  let email_error = Memo::new(move |_| {
    validated_email().and_then(|r| {
      match r.map(|e| models::validate_reasonable_email_address(e.as_ref())) {
        Ok(true) => None,
        Ok(false) => Some(Either::Left(
          "Though technically valid, this email address is probably not \
           correct",
        )),
        Err(models::EmailAddressError::PredicateViolated) => {
          Some(Either::Right("Invalid email address"))
        }
        Err(models::EmailAddressError::LenCharMaxViolated) => {
          Some(Either::Right("Email is too long"))
        }
      }
    })
  });
  let confirm_email_error = Memo::new(move |_| {
    validated_confirm_email().and_then(|r| r.then_some("Emails do not match"))
  });

  // views for input errors
  let name_error_view = move || name_error().map(error_view);
  let email_error_view =
    move || email_error().map(|e| e.either(warning_view, error_view));
  let confirm_email_error_view = move || confirm_email_error().map(error_view);

  // hint values for the input fields
  let name_input_hint =
    Signal::derive(move || name_error().map(|_| FieldHint::Error));
  let email_input_hint = Signal::derive(move || {
    email_error().map(|e| match e {
      Either::Left(_) => FieldHint::Warning,
      Either::Right(_) => FieldHint::Error,
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
        <div class="flex flex-col gap-1">
          <label class="" for="name">"Full Name"</label>
          <Field size={FieldSize::Large} hint={name_input_hint} {..}
            placeholder="Enter your full name" id="name"
            on:input=write_name_callback prop:value=read_name_callback
          />
          { name_error_view }
        </div>

        <div class="flex flex-col gap-1">
          <label class="" for="email">"Email"</label>
          <Field size={FieldSize::Large} hint={email_input_hint} {..}
            placeholder="Enter your email" type="email" id="email"
            on:input=write_email_callback prop:value=read_email_callback
          />
          { email_error_view }
        </div>

        <div class="flex flex-col gap-1">
          <label class="" for="confirm_email">"Confirm Email"</label>
          <Field size={FieldSize::Large} hint={confirm_email_input_hint} {..}
            placeholder="Enter your email again" type="email" id="confirm_email"
            on:input=write_confirm_email_callback prop:value=read_confirm_email_callback
          />
          { confirm_email_error_view }
        </div>
      </form>

      <div class="flex flex-row">
        <div class="flex-1" />
        <Button size={ButtonSize::Large} {..} on:click={move |_| {action.dispatch(());}}>
          "Sign up"
          <lsc::icons::ArrowRightIcon {..} class="size-5" />
        </Button>
      </div>

      { move || action_value_view().map(|v| view! {
        <div class="self-center mt-4">{ v }</div>
      })}
    </FloatingBoxSection>
  }
}

#[server(name = SignupActionParams)]
async fn signup(
  name: String,
  email: String,
  confirm_email: String,
) -> Result<UserRecordId, ServerFnError> {
  use auth_domain::{AuthDomainService, AuthSession, DynAuthDomainService};
  use models::{UserAuthCredentials, UserCreateRequest};

  let auth_service =
    use_context::<DynAuthDomainService>().ok_or_else(|| {
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
  if !models::validate_reasonable_email_address(email.as_ref()) {
    return Err(ServerFnError::new("Email is unreasonable"));
  }
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
