#[derive(Debug)]
struct Tarefa {
    titulo: String,
    status: Status,
}

#[derive(Debug)]
enum Status {
    Pendente,
    Concluida,
}

use std::io;

fn main() {
    let mut tarefas: Vec<Tarefa> = Vec::new();

    loop {
        println!("\n--- MENU ---");
        println!("1 - Adicionar tarefa");
        println!("2 - Listar tarefas");
        println!("3 - Marcar tarefa como concluída");
        println!("4 - Remover tarefa");
        println!("5 - Sair");

        let opcao = ler_numero();

        match opcao {
            1 => adicionar(&mut tarefas),
            2 => listar(&tarefas),
            3 => marcar(&mut tarefas),
            4 => remover(&mut tarefas),
            5 => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}

fn ler_numero() -> i32 {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler");
    entrada.trim().parse().unwrap_or(-1)
}

fn adicionar(tarefas: &mut Vec<Tarefa>) {
    println!("Digite o título da tarefa:");
    let mut titulo = String::new();
    io::stdin().read_line(&mut titulo).unwrap();
    let titulo = titulo.trim().to_string();

    let nova = Tarefa {
        titulo,
        status: Status::Pendente,
    };

    tarefas.push(nova);
    println!("Tarefa adicionada!");
}

fn listar(tarefas: &Vec<Tarefa>) {
    if tarefas.is_empty() {
        println!("Nenhuma tarefa encontrada.");
        return;
    }

    println!("\nTarefas:");
    for (i, t) in tarefas.iter().enumerate() {
        println!("{} - {:?} - {}", i, t.status, t.titulo);
    }
}

fn marcar(tarefas: &mut Vec<Tarefa>) {
    listar(tarefas);

    println!("Digite o número da tarefa:");
    let indice = ler_numero() as usize;

    if let Some(t) = tarefas.get_mut(indice) {
        t.status = Status::Concluida;
        println!("Tarefa marcada como concluída!");
    } else {
        println!("Índice inválido!");
    }
}

fn remover(tarefas: &mut Vec<Tarefa>) {
    listar(tarefas);

    println!("Digite o número da tarefa para remover:");
    let indice = ler_numero() as usize;

    if indice < tarefas.len() {
        tarefas.remove(indice);
        println!("Tarefa removida!");
    } else {
        println!("Índice inválido!");
    }
}
