use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::signer_account_id;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

// **************** Estructura de Meme *********************************
//
// 2.  Clases necesarias para el contrato
// Implementamos serde cuando necesitamos retornar la estructura serializada a JSON util en NEAR
// CLI y frotend. En este caso se utilizan ambas serializaciones ya que sera utilizado borsh en la
// serialización y deserialización del contrato en la blockchain de NEAR
//
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Meme {
    pub id: u64,
    pub creado_por: String,
    pub titulo: String,
    pub museo: String,
    pub url: String,
    pub donaciones: u128,
}

// Implemetación del trait Default para inicializar la estructura de Meme 
impl Default for Meme {
    fn default() -> Self {
        Meme {
            id: 0,
            creado_por: String::from(""),
            titulo: String::from(""),
            museo: String::from(""),
            url: String::from(""),
            donaciones: 0,
        }
    }
}

// Implementación del metodo new que permitira crear nuevos memes 
impl Meme {
    pub fn new(titulo: String, url: String, museo: String) -> Self {
        Self {
            //id: 0,
            //creado_por: String::from(""),
            id: env::block_index(),
            creado_por: env::signer_account_id(),
            titulo,
            museo,
            url,
            donaciones: 0,
        }
    }
}

//1. estructura,  state del contrato y inicialización de los valores por default
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleMemeMuseum {
    //Guardamos solo los ID para evitar tener que editar en ambos lugares cuando se modifique un meme.
    museos: UnorderedMap<String, Vec<u64>>,
    memes: UnorderedMap<u64, Meme>,
}

//inicializamos el state del contrato
impl Default for SimpleMemeMuseum {
    fn default() -> Self {
        Self {
            //inicializamos las colecciones 
            museos: UnorderedMap::new(b"u".to_vec()),
            memes: UnorderedMap::new(b"e".to_vec()),
        }
    }
}


// **************** Métodos ****************

#[near_bindgen]
impl SimpleMemeMuseum {
    //Función publica y de contrato mutable que crea un meme y lo almacena en las colecciones 
    pub fn crear_meme(&mut self, titulo: String, url: String, nombre_museo: String) {
        //Creamos el objeto del meme
        let meme = Meme::new(
            String::from(&titulo),
            String::from(&url),
            String::from(&nombre_museo),
        );

        //Lo guardamos en la coleccion de memes
        self.memes.insert(&meme.id, &meme);

        //Buscamos si el museo existe para despues añadir el meme en el. La función get retornara
        //el valor de un key dado como un Option<Vec<64>> con el vector de museos 
        let museo = self.museos.get(&nombre_museo);

        //Si existe el museo is_some() retornara un true y agregamos el nuevo id de ese meme  
        if museo.is_some() {
            //m almacena el valor del Some() del Option<> o da un panic en caso de ser Null
            let mut m = museo.unwrap();
            // m al ser un vector hacemos un push para almacenar el dato
            m.push(meme.id);
    
            //insertamos en la colección de museos el museo y el meme ligado a el
            self.museos.insert(&nombre_museo, &m);
        }
        //Si no existe, creamos un nuevo museo, le agregamos el meme y lo guardamos.
        else {
            let mut nuevo_museo = Vec::new();

            nuevo_museo.push(meme.id);
            self.museos.insert(&nombre_museo, &nuevo_museo);
        }

        //Manda un mensaje a la terminal al ejecutar el método
        env::log(
            format!(
                "Nuevo meme añadido con éxito. Museo: {}, Id Meme: {}",
                &nombre_museo, meme.id
            )
            .as_bytes(),
        )
    }

    //Metodo Read Only que retorna un Option<Meme> al consultar la colección de memes pasando como
    //parametro el id del meme
    pub fn obtener_meme(&self, id: u64) -> Option<Meme> {
        self.memes.get(&id)
    }

    //Metodo Read only que retorna un vector con los datos de la colección de memes
    pub fn obtener_lista_memes(&self) -> Vec<(u64, Meme)> {
        self.memes.to_vec()
    }

    //Metodo Read only que retorna la lista de museos como un Vector de strings, para esto solo
    //toma las keys de la colección de museos y la convierte a un vector
    pub fn obtener_lista_museos(&self) -> Vec<String> {
        self.museos.keys_as_vector().to_vec()
    }

    //Regresamos un Vector con los memes que tiene ese museo.
    //Metodo Read only que retorna un vector con el junto de memes
    pub fn obtener_memes_museo(&self, nombre_museo: String) -> Vec<Meme> {
        //Obtenemos el museo como un Option<Vec<64>>
        let museo = self.museos.get(&nombre_museo);

        //si el museo existe
        if museo.is_some() {
            //creamos un vector que almacene la lista de memes en el museo 
            let mut lista_memes = Vec::new();

            //el for  recorre cada elemento en el museo
            for meme in &museo.unwrap() {
                //obtenemos el meme mediante su id
                let m = self.memes.get(meme);

                //si el meme existe
                if m.is_some() {
                    //mandamos el meme al vector de lista de memes
                    lista_memes.push(m.unwrap());
                }
            }
            //retornamos la lista de memes
            lista_memes
        // si no existe el museo
        } else {
            //creamos el vector vacio para memes
            Vec::new()
        }
    }

    
}

