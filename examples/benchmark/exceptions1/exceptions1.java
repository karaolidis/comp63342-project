class A extends Throwable {
}

class B extends A {
}

class C extends B {
}

class D extends C {
}

public class exceptions1 {
  public static void test() {
    D d = new D();
    C c = new C();
    B b = new B();
    A a = new A();
    A e = a;
    try {
      throw e;
    } catch (A exc) {
      assert false;
    }
  }
}
