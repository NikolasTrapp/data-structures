use std::fmt;
use std::io;

#[derive(Clone)]
struct Node {
    value: i32,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            right: None,
            left: None
        }
    }

    fn print(&mut self) {
        if let Some(next) = &mut self.left {
            next.print();
        }
        println!("{}", self.value);
        if let Some(next) = &mut self.right {
            next.print();
        }
    }

    fn get_depth(&mut self, number: i32) -> u32 {
        let mut count = 0;
        if number == self.value {
            return 0
        }

        if number < self.value {
            if let Some(node) = &mut self.left {
                count = node.get_depth(number);
                return count + 1
            }
        } else {
            if let Some(node) = &mut self.right {
                count = node.get_depth(number);
                return count + 1
            }
        }
        return count
    }

    fn get_min(&mut self) -> &mut Node {
        if self.left.is_some() {
            return self.left.as_mut().unwrap().get_min();
        } else {
            self
        }
    }

    fn get_max(&mut self) -> &mut Node {
        if self.right.is_some() {
            return self.right.as_mut().unwrap().get_max();
        } else {
            self
        }
    }

    fn insert(&mut self, number: i32) {
        if self.value == number {
            return
        }
        if number < self.value {
            if let Some(node) = &mut self.left {
                node.insert(number);
            } else {
                self.left = Some(Box::new(Node::new(number)));
            }
        } else {
            if let Some(node) = &mut self.right {
                node.insert(number);
            } else {
                self.right = Some(Box::new(Node::new(number)));
            }
        }
    }

    fn delete(&mut self, number: i32) -> Option<Box<Node>> {
        if number < self.value {
            if let Some(node) = &mut self.left {
                self.left = node.delete(number);
            }
        } else if number > self.value {
            if let Some(node) = &mut self.right {
                self.right = node.delete(number);
            }
        } else {
            if self.left.is_none() && self.right.is_none() {
                return None;
            } else if self.left.is_none() {
                return self.right.take()
            } else if self.right.is_none() {
                return self.left.take()
            } else {
                let mut temp = self.right.clone();
                let mut left_most = temp.unwrap().get_min().value;
                self.value = left_most;
                self.right = self.right.take().and_then(|mut node| node.delete(left_most))
            }
        }
        Some(Box::new(self.clone()))
    }


}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let mut root: Option<Box<Node>> = None;
    loop {
        print_menu();
        let option = get_int_input();

        match option {
            1 => add_node(&mut root),
            2 => remove_node(&mut root),
            3 => print_all(&mut root),
            4 => print_higher(&mut root),
            5 => print_lower(&mut root),
            6 => root = Some(Box::new(Node::new(get_int_input()))),
            7 => get_depth_from_node(&mut root),
            8 => break,
            _ => println!("Digite uma opção válida")
        }
    }

}

fn print_menu() {
    println!("-=-=-=-=-=-=-MENU-=-=-=-=-=-=-");
    println!("| 1 - Inserir um Node     |");
    println!("| 2 - Remover um Node     |");
    println!("| 3 - Imprimir os Nodes   |");
    println!("| 4 - Obter maior node    |");
    println!("| 5 - Obter menor node    |");
    println!("| 6 - Criar a árvore      |");
    println!("| 7 - Obter profundidade  |");
    println!("| 8 - Sair                |");
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
    println!("O que deseja fazer:");
}

fn add_node(root: &mut Option<Box<Node>>) {
    match root {
        None => println!("Considere criar uma árvore!"),
        Some(ref mut node) => {
            println!("Qual o valor do novo node:");
            let value = get_int_input();
            node.insert(value);
        }
    };

}

fn remove_node(root: &mut Option<Box<Node>>) {
    match root {
        None => println!("Considere criar uma árvore!"),
        Some(ref mut node) => {
            println!("Qual o valor do node que deseja remover:");
            let value = get_int_input();
            node.delete(value);
        }
    };
}

fn print_all(root: &mut Option<Box<Node>>) {
    match root {
        None => println!("Considere criar uma árvore!"),
        Some(node) => {
            node.print();
        },
    };
}

fn print_lower(root: &mut Option<Box<Node>>) {
    match root {
        None => println!("Considere criar uma árvore!"),
        Some(node) => {
            node.get_min();
        }
    };
}

fn print_higher(root: &mut Option<Box<Node>>) {
    match root {
        None => println!("Considere criar uma árvore!"),
        Some(node) => {
            node.get_max();
        }
    };
}

fn get_depth_from_node(root: &mut Option<Box<Node>>) {
    match root {
        None => println!("Considere criar uma árvore!"),
        Some(ref mut node) => {
            println!("Qual o valor do node que deseja pegar a profundidade:");
            let value = get_int_input();
            node.get_depth(value);
        }
    };
}

fn get_int_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Não alopra né amigão");
    let number: i32 = input.trim().parse().expect("Invalid input");
    // let n: i32 = input.trim().parse().expect("Po parça, custa digitar um número que nem gente?");
    number
}
