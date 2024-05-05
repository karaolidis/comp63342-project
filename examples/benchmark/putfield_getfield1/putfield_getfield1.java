class A {
  public int i;
}

public class putfield_getfield1 {
  public static void test(int value) {
    A a = new A();
    a.i = value;
    assert value == a.i;
  }
}
