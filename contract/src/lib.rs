use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise, AccountId, collections::UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};


// const DEFAULT_NUM_COLABORACIONES: U8 = 0;

//struct principal.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    colaboradores: UnorderedMap<String, Colaborador>,
}

//Inicializamos el contrato por default
impl Default for Contract {
    fn default() -> Self {
        Self {
            //Inicializamos la colección con un prefijo único
            colaboradores: UnorderedMap::new(b"p".to_vec()),
        }
    }
}

//Structs dentro del contrato
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Colaborador {
    pub cuenta: String,
    pub dataset_id: u64,
    pub num_colaboraciones: u8,
    pub porcentaje_validado:u8,
    pub reconpensa: bool,
    pub castigo: bool,
}

//En este contrato no se utiliza el default, pero es buena práctica tenerlo inicializado.
impl Default for Colaborador {
    fn default() -> Self {
        Colaborador { 
            cuenta: String::from(""), 
            dataset_id: 0, 
            num_colaboraciones: 0,
            porcentaje_validado:0,
            reconpensa: false, 
            castigo: false 
        } 
    }
}

//Creamos la implementación del método new. El equivalente en AS sería el constructor.
impl Colaborador {
    pub fn new(cuenta: String, dataset_id: u64) -> Self {
        Self {
            cuenta,
            dataset_id,
            num_colaboraciones: 0,
            porcentaje_validado: 0,
            reconpensa: false,
            castigo: false,
        }
    }
}

//Igual que con el struct de Participante, implementamos los métodos del contrato en un impl.
#[near_bindgen]
impl Contract {

    pub fn set_colaborador(&mut self, dataset_id: u64) {
        let cuenta = env::signer_account_id().to_string();

        assert!(dataset_id > 0, "ID de dataset inválido.");

        let colabordor = Colaborador::new(cuenta.clone(), dataset_id);

        self.colaboradores.insert(&cuenta, &colabordor);

        env::log_str("Registro creado exitosamente.");
    }

    pub fn get_colaborador(&self, cuenta: String) -> Option<Colaborador> {
        self.colaboradores.get(&cuenta)
    }

    pub fn get_colaboradores(&self) -> Vec<Colaborador> {
        self.colaboradores.values_as_vector().to_vec()
    }

    /* 
    Este método permite actualizar el porcentaje de validación que va alcanzando el colaborador. Cuando se tenga 3 colaboraciones posteriores de otros usuarios en el mismo dataset_id, el método evaluar si el colaborador pasó o no el proceso de validación, de acuerdo a ello procede el castigo o recompensa 
    */
    pub fn set_validacion(&mut self, cuenta:String, porc_validado:u8) {
        let master: AccountId = "eber.testnet".parse().unwrap();

        assert!(
            env::signer_account_id() == master,
            "No eres el administrador, no puedes realizar la validación"
        );

        let cuenta_colaborador: AccountId = cuenta.parse().unwrap();

        match self.colaboradores.get(&cuenta) {
            Some(mut colaborador) => {
                // verifica si el colaborador ya ha concluido el proceso de validación
                if colaborador.reconpensa == true || colaborador.castigo == true {
                    env::log_str("El colaborador ya concluyó la validación, puede colaborar con otro dataset")
                } else {
                    colaborador.porcentaje_validado = porc_validado;
                    colaborador.num_colaboraciones += 1;
                    
                    if colaborador.num_colaboraciones == 3 {
                        if colaborador.porcentaje_validado >= 90 {
                            // recompensa
                            Promise::new(cuenta_colaborador).transfer(10 as u128);
                            env::log_str("El colaborador pasó la validación, se desbloqueará los NEAR en staking y se transfiere 10 NEAR");
                            colaborador.reconpensa = true;
                        } else {
                            // castigo
                            env::log_str("El colaborador NO pasó la validación, pierde los NEAR en staking");
                            colaborador.castigo = true;
                        }
                    }
                    
                    self.colaboradores.insert(&cuenta, &colaborador);
                    env::log_str("Validación completada");
                }

                // true
            }
            None => {
                env::log_str("La cuenta ingresada no es el de un colaborador");

                // false
            }
        }
    }
}   



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_default_greeting() {
//         let contract = Contract::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(
//             contract.get_greeting(),
//             "Hello".to_string()
//         );
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let mut contract = Contract::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(
//             contract.get_greeting(),
//             "howdy".to_string()
//         );
//     }
// }
