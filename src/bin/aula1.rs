fn main() {
    // roda com cargo run src/bin/aula1.rs ou cargo run --bin aula1
    //vamos criar aqui a aula de vec (vetores) assim fica melhor pra mim entender se caso euq uiser revisar depois

    // isso cria uma lista vazia, o mut indica que a lista pode ser modificada
    let mut lista: Vec<i32> = Vec::new();

    // imprime a lista vazia
    println!("Lista inicial: {:?}", lista);

    //adiciona elementos na lista
    lista.push(10);
    lista.push(20);
    lista.push(30);
    println!("essa é a lista: {:?}", lista);
    // esse :? é usado para imprimir o vetor de forma legível ( na vdd eu não entendo, mas sei que serve pra isso)

    // mas da pra criar uma lita com valores, pelas minhas pesquisas ja adiciona o valor na vareavel lista
    // mas como ja tem a vareavel lista criada, vou criar a lista2
    // que coisa chata estudar rust, não tem curso nessa porcaria
    
    let mut lista2 = vec![1, 2, 3, 4, 5];   // lista de inteiros
    let lista3: Vec<&str> = vec!["fruta" , "legume", "verdura"]; // lista de strings

    println!("essa é a lista2: {:?}", lista2); // aqui vai printar a lista2 com os numeros
    println!("essa é a lista3: {:?}", lista3); // aqui vai printar a lista3 com strings

    // mas se usar o mut na lista2 ou lista3, da erro
    // mas pdoe usar o mut na lista, mas so se vc for mudar o valores da lista
    // por exemplo, coloquei mut na lista2, pq vou mudar os valores dela

    lista2.push(6); // adiciona o valor 6 na lista2

    println!("essa é a lista2 depois de adicionar o 6: {:?}", lista2);

    // a porcaria do rust não deixa remover um valor que não existe na lista (obvio)
    // e se colocar mut e não mudar o valor depois da erro tbm
    lista2.remove(0); // remove o primeiro valor da lista2 (indice 0)

    println!("essa é a lista2 depois de remover o primeiro valor: {:?}", lista2);

    //  oxe da pra remover o ultimo valor da lista2 com pop ou escolher o indice com o nome.remove(indice) o indice é do 0 endiante
    // tanta coisa pra aprender, mas to tentando essa leseira de rust

    //----------------------------------------------------------------------------------------------------------------

    //aqui eu posso usar o match pra ver se o valor existe na lista2, assim com o match e o get
    // que usar assim match nomedavareavel.get(indice) onde o indice é o numero do indice que eu quero ver se existe
    //assim se por acaso colocar um indice que não existe, ele não da erro, ele so entra no None
    match lista2.get(1) {
    Some(valor) => println!("Valor encontrado: {}", valor),
    None => println!("Não existe essa posição"),
    }

    // esse aqui vai dar None pq o indice 10 não existe na lista2
    match lista2.get(10) {
    Some(valor) => println!("Valor encontrado: {}", valor),
    None => println!("Não existe essa posição"),
    }

    // da pra fazer essa mesma coisa com if e else
    if let Some(valor) = lista2.get(2) {
        println!("Valor encontrado com if let: {}", valor);
    } else {
        println!("Não existe essa posição com if let");
    }

    // agora eu me pergunto qual melhor usar, o match ou o if let, não sei nem oq é same pq nunca vi isso antes

    // ----------------------------------------------------------------------------------------------------------------

    // basicamente ele tem que responder isso no terminal

    //Lista inicial: []
    //essa é a lista: [10, 20, 30]
    //essa é a lista2: [1, 2, 3, 4, 5]
    //essa é a lista3: ["fruta", "legume", "verdura"]
    //essa é a lista2 depois de adicionar o 6: [1, 2, 3, 4, 5, 6]
    //essa é a lista2 depois de remover o primeiro valor: [2, 3, 4, 5, 6]
    //Não existe essa posição

}