/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> // Renamed to UpperCamelCase
{
	// Use q1 as the primary queue for enqueueing and holding elements.
	// q2 is used as temporary storage during pop.
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> MyStack<T> { // Renamed to UpperCamelCase
    pub fn new() -> Self {
        Self {
			// Initialize both queues
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // Push element onto the main queue (q1)
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // If q1 is empty, the stack is empty
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // Move all elements except the last one from q1 to q2
        while self.q1.size() > 1 {
            // unwrap is safe here because we know q1 is not empty
            self.q2.enqueue(self.q1.dequeue().unwrap());
        }

        // The last element in q1 is the element to pop
        // Dequeue the last element. Unwrap is safe because we checked size > 0.
        let popped_element = self.q1.dequeue().unwrap();

        // Swap q1 and q2 so q1 holds the remaining elements for the next operation.
        // This swap must happen *after* the borrow from dequeue() is finished.
        std::mem::swap(&mut self.q1, &mut self.q2);

        Ok(popped_element) // Return the popped element wrapped in Ok
    }
    pub fn is_empty(&self) -> bool {
		      // The stack is empty if the main queue (q1) is empty
		      self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = MyStack::<i32>::new(); // Use renamed struct
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}