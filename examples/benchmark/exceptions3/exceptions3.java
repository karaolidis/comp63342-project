class A extends Throwable {
}

class B extends A {
}

class C extends B {
}

public class exceptions3 {
  public static void test() {
    throw new B();
  }
}
