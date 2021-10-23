use std::fmt::{Display, Formatter};
use std::error::Error;

use crate::entity::{Role, User};

use super::DiscordGateway;

#[derive(Debug)]
pub(crate) enum RequestResultPolicy {
    Success,
    NotFound,
    Fails
}

#[derive(Debug)]
pub(crate) struct MockedFailure;

impl Error for MockedFailure {}
impl Display for MockedFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mocked failure")
    }
}

pub(crate) struct MockDiscordGateway;

type RequestReturn = Result<Option<User>, MockedFailure>;

impl DiscordGateway for MockDiscordGateway {
    type E = MockedFailure;

    fn request(&self, user_id: &str) -> RequestReturn {
        self.request_with_policy(user_id, RequestResultPolicy::Success)
    }
}

impl MockDiscordGateway {
    fn request_with_policy(&self, user_id: &str, policy: RequestResultPolicy) -> RequestReturn {
        println!("[i] User with {} created with policy {:?}.", user_id, policy);

        match policy {
            RequestResultPolicy::Success => Ok(Some(
                User {
                    name: format!("User #{}", user_id),
                    nickname: format!("Mr. {}", user_id),
                    pic_url: format!("somewhere://{}", user_id),
                    roles: vec![
                        Role { name: format!("Role for user {}", user_id), color_code: 0 }
                    ]
                }
            )),
            RequestResultPolicy::NotFound => Ok(None),
            RequestResultPolicy::Fails => Err(MockedFailure)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{MockDiscordGateway, RequestResultPolicy};

    #[test]
    fn sucess_when_policy_says_so_and_it_returns_value_in_the_format_that_i_want() {
        let gateway = MockDiscordGateway;
        let user = gateway.request_with_policy("12345", RequestResultPolicy::Success);

        let user = user.expect("It should success");
        let user = user.expect("It should have a value");

        assert_eq!(user.name, "User #12345");
        assert_eq!(user.nickname, "Mr. 12345");
        assert_eq!(user.pic_url, "somewhere://12345");
        assert_eq!(user.roles[0].name, "Role for user 12345");
        assert_eq!(user.roles[0].color_code, 0);
    }
}
