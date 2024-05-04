class A extends RuntimeException {
}

class B extends A {
}

class C extends B {
}

class F {
  void foo() {
    B b = new B();
    throw b;
  }
}

public class exceptions13 {

  public void test() {
    F f = new F();
    try {
      f.foo();
    } catch (B exc) {
      assert false;
    }
  }
}
