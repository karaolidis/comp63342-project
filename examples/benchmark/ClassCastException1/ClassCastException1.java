public class ClassCastException1 {
  public static void test(Object x) {
    try {
      String y = (String) x;
    } catch (ClassCastException exc) {
      assert false;
    }
  }
}
