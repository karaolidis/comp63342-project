class A {
}

class B {
}

public class ClassCastException3 {
  public static void test() {
    Object a = new A();
    B b = (B) a;
  }
}
