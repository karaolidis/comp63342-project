public class StringValueOf07 {
  public static void test(long longValue) {
    System.out.printf("long = %s\n", String.valueOf(longValue));
    String tmp = String.valueOf(longValue);
    assert tmp.equals("100000000000");
  }
}
