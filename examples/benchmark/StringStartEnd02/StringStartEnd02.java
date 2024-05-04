public class StringStartEnd02 {
  public static void test(String[] strings) {
    int i = 0;
    for (String string : strings) {
      if (string.startsWith("te")) ++i;
    }
    assert i == 1;
  }
}
