use std::ptr;
use std::fmt;
struct Node
{
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList
{
    head: Option<Box<Node>>
}

impl LinkedList{

    pub fn new() -> Self{
        LinkedList{head:None}
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        let mut current = &self.head;
        for _ in 0..index {
            if let Some(next) = current {
                current = &next.next;
            } else {
                return None; // Early return if the index is out of bounds
            }
        }

        current.as_ref().map(|node| node.value)
    }

    pub fn update(&mut self, index: usize, newValue: i32) -> Result<(), String> {
        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(ref mut next) = current {
                current = &mut next.next;
            } else {
                return Err("Index out of bounds".to_string());
            }
        }

        if let Some(ref mut node) = current {
            node.value = newValue;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<i32> {
        if index == 0 {
            return self.head.take().map(|oldHead| {
                self.head = oldHead.next;
                oldHead.value
            });
        } else {
            let mut current = &mut self.head;
            for _ in 0..(index - 1) {
                if current.is_none() {
                    return None;
                }
                current = &mut current.as_mut().unwrap().next;
            }
    
            if let Some(ref mut node) = current {
                if let Some(mut nodeToRemove) = node.next.take() {
                    node.next = nodeToRemove.next.take();
                    return Some(nodeToRemove.value); 
                }
            }
        }
    
        None
    }
    


    pub fn insert(&mut self, value:i32,index:Option<i32> )
    {
        let mut newNode = Box::new(Node{
            value,
            next:None
        });

        let mut currentIndex=  0;
        if index.is_none()
        {
            if let Some(ref mut head) = self.head{
                let mut current = head;
                while let Some(ref mut next) = current.next
                {
                    current = next;
                }
                current.next = Some(newNode);
            }
            else
            {
                self.head = Some(newNode);
            }
        }
        else
        {
            let unwrappedIndex = index.unwrap();
            if unwrappedIndex==0
            {
                newNode.next = self.head.take();
                self.head = Some(newNode);
            }
            else
            {
                if let Some( ref mut head) = self.head
                {
                    let mut current = head;
                    let mut pos = 0;
                    while pos < unwrappedIndex-1 && current.next.is_some()
                    {
                        current = current.next.as_mut().unwrap();
                        pos+=1;
                    }
                    newNode.next = current.next.take();
                    current.next = Some(newNode);
                }
                else
                {
                    self.head = Some(newNode);
                }
            }
        }
    }
}


fn main() {
    let mut list  = LinkedList::new();
    list.insert(5,None);
    list.insert(10,Some(0));
    list.insert(15,Some(1));
    list.insert(20,None);

    println!("Deleted value: {:?}",list.delete(0));

    match list.update(2,25){
        Ok(_) => println!("Update successful"),
        Err(e) => println!("Error updating: {}", e),
    }

    
}
