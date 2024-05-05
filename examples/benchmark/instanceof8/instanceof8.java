public class instanceof8 {
  public static boolean test(Integer i) {
    if (i instanceof Integer) {
      return true;
    } else {
      return false;
    }
  }

  public static void test() {
    assert (!test(null));
    assert (test(new Integer(1)));
  }
}
