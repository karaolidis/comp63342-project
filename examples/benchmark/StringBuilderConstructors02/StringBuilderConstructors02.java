public class StringBuilderConstructors02 {
  public static void test(String arg) {
    StringBuilder buffer3 = new StringBuilder(arg);
    assert buffer3.toString().equals(arg);
  }
}
