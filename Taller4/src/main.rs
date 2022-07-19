fn main() {
    
}

struct Test{
    //numero de modelos integer
    numero_modelos: i32,
    //numero de preguntas integer
    numero_preguntas: i32,
    //Respuestas por modelo, vector de strings
    respuestas_por_modelo: Vec<String>,
    //valor de la respuesta correcta, integer
    valor_respuesta_correcta: i32,
    //valor de la respuesta incorrecta, integer
    valor_respuesta_incorrecta: i32,
}



enum Pregunta{
    A,
    B,
    C,
    D,
}