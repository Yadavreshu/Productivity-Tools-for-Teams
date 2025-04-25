#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Vec, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub assigned_to: String,
    pub completed: bool,
}

const TASK_COUNT: Symbol = symbol_short!("TASK_CNT");

#[contracttype]
pub enum TaskRegistry {
    Task(u64),
}

#[contract]
pub struct TeamProductivity;

#[contractimpl]
impl TeamProductivity {
    pub fn add_task(env: Env, title: String, assigned_to: String) -> u64 {
        let mut count = env.storage().instance().get(&TASK_COUNT).unwrap_or(0);
        count += 1;

        let task = Task {
            id: count,
            title,
            assigned_to,
            completed: false,
        };

        env.storage().instance().set(&TaskRegistry::Task(count), &task);
        env.storage().instance().set(&TASK_COUNT, &count);

        count
    }

    pub fn complete_task(env: Env, id: u64) {
        let mut task: Task = env.storage().instance().get(&TaskRegistry::Task(id)).expect("Task not found");
        task.completed = true;
        env.storage().instance().set(&TaskRegistry::Task(id), &task);
    }

    pub fn get_task(env: Env, id: u64) -> Task {
        env.storage().instance().get(&TaskRegistry::Task(id)).expect("Task not found")
    }

    pub fn total_tasks(env: Env) -> u64 {
        env.storage().instance().get(&TASK_COUNT).unwrap_or(0)
    }
}

