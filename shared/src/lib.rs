use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
type Uuid = String;

#[derive(Deserialize, Serialize)]
pub struct FantasiforsterInformation {
    pub id: Uuid,
    pub titel: String,
    pub innehåll: String,
    pub födelsedag: DateTime<Local>,
    pub uppfinnare_namn: String,
}

impl FantasiforsterInformation {
    pub fn producera(
        forster: Fantasiforster,
        uppfinnare_namn: String,
    ) -> FantasiforsterInformation {
        FantasiforsterInformation {
            id: forster.id,
            titel: forster.titel,
            innehåll: forster.innehåll,
            födelsedag: forster.födelsedag,
            uppfinnare_namn,
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Fantasiforster {
    pub id: Uuid,
    pub titel: String,
    pub innehåll: String,
    pub födelsedag: DateTime<Local>,
    pub uppfinnare_id: Uuid,
}

impl Fantasiforster {
    pub fn producera(
        id: Uuid,
        förfrågan: ProduceraFantasiforsterFörfrågan,
        uppfinnare_id: Uuid,
        födelsedag: DateTime<Local>,
    ) -> Fantasiforster {
        Fantasiforster {
            id,
            titel: förfrågan.titel,
            innehåll: förfrågan.innehåll,
            födelsedag,
            uppfinnare_id,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FantasiforsterFilter {}

impl FantasiforsterFilter {
    pub fn default() -> FantasiforsterFilter {
        FantasiforsterFilter {}
    }
}
#[derive(Deserialize)]
pub struct ProduceraFantasiforsterFörfrågan {
    titel: String,
    innehåll: String,
}

impl ProduceraFantasiforsterFörfrågan {
    pub fn skaffa_mig_din_titel(&self) -> &str {
        &self.titel
    }
    pub fn skaffa_mig_ditt_innehåll(&self) -> &str {
        &self.innehåll
    }
}
#[derive(Deserialize, Serialize)]
pub struct Hjärna {
    id: Uuid,
    hjärnannamn: String,
    födelsedag: DateTime<Local>,
    krypterade_lösenordet: String,
}

impl Hjärna {
    pub fn skaffa_mig_ditt_namn(&self) -> &str {
        &self.hjärnannamn
    }

    pub fn skaffa_mig_ditt_id(&self) -> &Uuid {
        &self.id
    }

    pub fn skaffa_mig_ditt_krypterade_lösenordet(&self) -> &str {
        &self.krypterade_lösenordet
    }

    pub fn registrera(
        id: Uuid,
        förfrågan: RegistreraHjärnaFörfrågan,
        födelsedag: DateTime<Local>,
        krypterade_lösenordet: String,
    ) -> Hjärna {
        Hjärna {
            id,
            hjärnannamn: förfrågan.hjärnannamn,
            födelsedag,
            krypterade_lösenordet,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct RegistreraHjärnaFörfrågan {
    hjärnannamn: String,
    lösenord: String,
    lösenord_extra: String,
}

impl RegistreraHjärnaFörfrågan {
    pub fn skaffa_mig_ditt_namn(&self) -> &str {
        &self.hjärnannamn
    }
    pub fn skaffa_mig_ditt_lösenord(&self) -> &str {
        &self.lösenord
    }

    pub fn skaffa_mig_ditt_lösenord_extra(&self) -> &str {
        &self.lösenord_extra
    }

    pub fn producera(
        hjärnannamn: String,
        lösenord: String,
        lösenord_extra: String,
    ) -> RegistreraHjärnaFörfrågan {
        RegistreraHjärnaFörfrågan {
            hjärnannamn,
            lösenord,
            lösenord_extra,
        }
    }

    pub fn validera(hjärnannamn: &str, lösenord: &str, lösenord_extra: &str) -> bool {
        !hjärnannamn.is_empty() && !lösenord.is_empty() && lösenord == lösenord_extra
    }
}

#[derive(Deserialize, Serialize)]
pub struct DemonstreraBesittarHjärnaFörfrågon {
    hjärnannamn: String,
    lösenord: String,
}
impl DemonstreraBesittarHjärnaFörfrågon {
    pub fn skaffa_mig_ditt_namn(&self) -> &str {
        &self.hjärnannamn
    }
    pub fn skaffa_mig_ditt_lösenord(&self) -> &str {
        &self.lösenord
    }

    pub fn producera(
        hjärnannamn: String,
        lösenord: String,
    ) -> DemonstreraBesittarHjärnaFörfrågon {
        DemonstreraBesittarHjärnaFörfrågon {
            hjärnannamn,
            lösenord,
        }
    }

    pub fn validera(hjärnannamn: &str, lösenord: &str) -> bool {
        !hjärnannamn.is_empty() && !lösenord.is_empty()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtInformation {
    pub hjärnannamn: String,
    pub id: Uuid,
}
