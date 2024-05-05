public class StringBuilderConstructors02 {
  public static void test(String input) {
    StringBuilder buffer3 = new StringBuilder(input);
    assert buffer3.toString().equals(input);
  }
}
