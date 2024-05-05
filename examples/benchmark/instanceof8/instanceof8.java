public class instanceof8 {
  public static boolean testb(Integer i) {
    if (i instanceof Integer) {
      return true;
    } else {
      return false;
    }
  }

  public static void test() {
    assert (!testb(null));
    assert (testb(new Integer(1)));
  }
}
