class A extends Throwable {
}

class B extends A {
}

class C extends B {
}

public class exceptions3 {
  public void test() {
    throw new B();
  }
}
