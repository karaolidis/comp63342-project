class A {
}

class B {
}

public class ClassCastException3 {
  public void test() {
    Object a = new A();
    B b = (B) a;
  }
}
