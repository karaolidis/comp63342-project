public class NullPointerException1 {
  public static void test(Object o) {
    try {
      o.hashCode();
      // should pass
      assert false;
    } catch (Exception e) {
    }
  }
}
