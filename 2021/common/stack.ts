export class Stack<T> {
  head: StackNode<T> | null = null;

  push(value: T) {
    let newNode = new StackNode<T>(value);
    newNode.previous = this.head;
    this.head = newNode;
  }

  peek(): T {
    if (this.head === null) {
      throw "Cannot peek an empty stack";
    }

    return this.head.value;
  }

  pop(): T {
    if (this.head === null) {
      throw "Cannot pop an empty stack";
    }

    let result = this.head.value;
    this.head = this.head.previous;

    return result;
  }

  drop() {
    this.head = null;
  }

  empty(): boolean {
    return this.head === null;
  }

  count(): number {
    if (this.head === null) {
      return 0;
    }

    return this.head.count();
  }
}

class StackNode<T> {
  previous: StackNode<T> | null = null;
  value: T;

  constructor(value: T) {
    this.value = value;
  }

  count(): number {
    return this.previous !== null ? 1 + this.previous.count() : 1;
  }
}
