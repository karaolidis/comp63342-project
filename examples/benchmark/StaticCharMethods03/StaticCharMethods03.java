public class StaticCharMethods03 {
  public static void test(String arg) {
    if (arg.length() < 1) return;

    char c = arg.charAt(0);
    assert Character.isDefined(c) == false;
  }
}
