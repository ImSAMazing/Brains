use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
pub type Uuid = String;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BrainInformation {
    id: Uuid,
    name: String,
    birthdate: DateTime<Local>,
}

impl BrainInformation {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
    pub fn create_from_brain(brain: Brain) -> BrainInformation {
        BrainInformation {
            id: brain.id,
            name: brain.brainname,
            birthdate: brain.birthdate,
        }
    }

    pub fn create(id: Uuid, name: String, birthdate: DateTime<Local>) -> BrainInformation {
        BrainInformation {
            id,
            name,
            birthdate,
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BrainfartInformation {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub birthdate: DateTime<Local>,
    pub mastermind_name: String,
    pub blew_minds: Vec<BrainInformation>,
    pub imploded_minds: Vec<BrainInformation>,
}

impl PartialEq for BrainfartInformation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl BrainfartInformation {
    pub fn create(
        brainfart: Brainfart,
        mastermind_name: String,
        blew_minds: Vec<BrainInformation>,
        imploded_minds: Vec<BrainInformation>,
    ) -> BrainfartInformation {
        BrainfartInformation {
            id: brainfart.id,
            title: brainfart.title,
            content: brainfart.content,
            birthdate: brainfart.birthdate,
            mastermind_name,
            blew_minds,
            imploded_minds,
        }
    }

    pub fn empty() -> BrainfartInformation {
        BrainfartInformation {
            id: Uuid::default(),
            title: String::default(),
            content: String::default(),
            birthdate: Local::now(),
            mastermind_name: String::default(),
            blew_minds: vec![],
            imploded_minds: vec![],
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NotifyAboutMindExplosionRequest {
    pub brainfart_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NotifyAboutMindImplosionRequest {
    pub brainfart_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Brainfart {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub birthdate: DateTime<Local>,
    pub mastermind_id: Uuid,
}

impl Brainfart {
    pub fn create(
        id: Uuid,
        request: CreateBrainfartRequest,
        mastermind_id: Uuid,
        birthdate: DateTime<Local>,
    ) -> Brainfart {
        Brainfart {
            id,
            title: request.title,
            content: request.content,
            birthdate,
            mastermind_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BrainfartFilter {}

impl BrainfartFilter {
    pub fn default() -> BrainfartFilter {
        BrainfartFilter {}
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBrainfartRequest {
    title: String,
    content: String,
}

impl CreateBrainfartRequest {
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn validate(title: &str, innehall: &str) -> bool {
        !title.is_empty() && !innehall.is_empty()
    }

    pub fn create(title: String, content: String) -> CreateBrainfartRequest {
        CreateBrainfartRequest { title, content }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Brain {
    id: Uuid,
    brainname: String,
    birthdate: DateTime<Local>,
    encrypted_password: String,
}

impl Brain {
    pub fn get_name(&self) -> &str {
        &self.brainname
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_encrypted_password(&self) -> &str {
        &self.encrypted_password
    }

    pub fn register(
        id: Uuid,
        request: RegisterBrainRequest,
        birthdate: DateTime<Local>,
        encrypted_password: String,
    ) -> Brain {
        Brain {
            id,
            brainname: request.brainname,
            birthdate,
            encrypted_password,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterBrainRequest {
    brainname: String,
    password: String,
    password_extra: String,
}

impl RegisterBrainRequest {
    pub fn get_name(&self) -> &str {
        &self.brainname
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_password_extra(&self) -> &str {
        &self.password_extra
    }

    pub fn create(
        brainname: String,
        password: String,
        password_extra: String,
    ) -> RegisterBrainRequest {
        RegisterBrainRequest {
            brainname,
            password,
            password_extra,
        }
    }

    pub fn validate(brainname: &str, password: &str, password_extra: &str) -> bool {
        !brainname.is_empty() && !password.is_empty() && password == password_extra
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProveOwnsBrainRequest {
    brainname: String,
    password: String,
}
impl ProveOwnsBrainRequest {
    pub fn get_name(&self) -> &str {
        &self.brainname
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn create(brainname: String, password: String) -> ProveOwnsBrainRequest {
        ProveOwnsBrainRequest {
            brainname,
            password,
        }
    }

    pub fn validate(brainname: &str, password: &str) -> bool {
        !brainname.is_empty() && !password.is_empty()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtInformation {
    pub brainname: String,
    pub id: Uuid,
}
