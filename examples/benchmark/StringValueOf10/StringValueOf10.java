public class StringValueOf10 {

  public static void test(String arg) {
    Object objectRef = arg; // assign string to an Object reference
    String tmp = String.valueOf(objectRef);
    assert tmp.equals(arg + "s");
  }
}
