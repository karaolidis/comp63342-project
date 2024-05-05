public class StaticCharMethods02 {
  public static void test(String arg) {
    if (arg.length() < 1) return;
    char c = arg.charAt(0);
    assert Character.toUpperCase(c) != Character.toLowerCase(c);
  }
}
