public class TokenTest02 {
  public static void test(String sentence) {
    String[] tokens = sentence.split(" ");

    int i = 0;
    for (String token : tokens) {
      if (i == 3) assert token.equals("genneration");
      ++i;
    }
  }
}
