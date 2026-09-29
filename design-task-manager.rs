// https://leetcode.com/problems/design-task-manager

use std::collections::HashMap;

type Task = (i32, i32, i32);

#[derive(Default)]
struct TaskManager {
    tasks: HashMap<i32, Task>,
    priorities: HashMap<i32, HashMap<i32, ()>>, 
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {

    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut ints = TaskManager::default();

        for task in tasks {
            ints.add(task[0], task[1], task[2])
        }

        ints
    }
    
    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.insert(task_id, (user_id, task_id, priority));
        self.insert_priority(priority, task_id); 
    }

    fn insert_priority(&mut self, priority: i32, task_id: i32) {
        let mut relation = self.priorities.entry(priority).or_insert(HashMap::new());
        relation.insert(task_id, ());
    }

    fn remove_priority(&mut self, task_id: &i32, priority: &i32) {
        if let Some(relation) = self.priorities.get_mut(priority) {
            relation.remove(task_id);
            if relation.is_empty() {
                self.priorities.remove(priority);
            }
        }
    }
    
    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let task = self.tasks.get_mut(&task_id).unwrap();
        let old_priority = task.2.clone();
        task.2 = new_priority;
        self.remove_priority(&task_id, &old_priority);
        self.insert_priority(new_priority, task_id); 
    }
    
    fn rmv(&mut self, task_id: i32) {
        let task = self.tasks.remove(&task_id).unwrap();
        
        self.remove_priority(&task.1, &task.2);
    }
    
    fn exec_top(&mut self) -> i32 {
        if let Some(priority) = self.priorities.keys().max() {
            let relation = self.priorities.get(priority).unwrap();
            let task_id = relation.keys().max().unwrap();
            let user_id = self.tasks.get(task_id).unwrap().0;

            self.rmv(*task_id);

            user_id
        } else {
            -1
        }
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */

 /////////////////////////////////////////////////////////////// AI



 use std::collections::{HashMap, BTreeSet};

struct TaskManager {
    tasks: HashMap<i32, (i32, i32)>,       // taskId -> (userId, priority)
    sorted_tasks: BTreeSet<(i32, i32)>,    // (priority, taskId) for automatic sorting
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut tm = TaskManager {
            tasks: HashMap::new(),
            sorted_tasks: BTreeSet::new(),
        };
        for t in tasks {
            tm.add(t[0], t[1], t[2]);
        }
        tm
    }
    
    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.insert(task_id, (user_id, priority));
        self.sorted_tasks.insert((priority, task_id));
    }
    
    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some((_, old_priority)) = self.tasks.get_mut(&task_id) {
            // Remove old entry from sorted set and insert updated one
            self.sorted_tasks.remove(&(*old_priority, task_id));
            *old_priority = new_priority;
            self.sorted_tasks.insert((new_priority, task_id));
        }
    }
    
    fn rmv(&mut self, task_id: i32) {
        if let Some((_, priority)) = self.tasks.remove(&task_id) {
            self.sorted_tasks.remove(&(priority, task_id));
        }
    }
    
    fn exec_top(&mut self) -> i32 {
        // BTreeSet stores elements ascendingly, so the highest priority/taskId is at the back
        if let Some(&(priority, task_id)) = self.sorted_tasks.iter().next_back() {
            self.sorted_tasks.remove(&(priority, task_id));
            let (user_id, _) = self.tasks.remove(&task_id).unwrap();
            user_id
        } else {
            -1
        }
    }
}