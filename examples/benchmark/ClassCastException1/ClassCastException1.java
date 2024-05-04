public class ClassCastException1 {
  public static void test(Object x) {
    String y = (String) x;
    assert false;
  }
}
