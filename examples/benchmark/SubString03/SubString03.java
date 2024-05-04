/*
 * Origin of the benchmark:
 *     license: 4-clause BSD (see /java/jbmc-regression/LICENSE)
 *     repo: https://github.com/diffblue/cbmc.git
 *     branch: develop
 *     directory: regression/jbmc-strings/SubString03
 * The benchmark was taken from the repo: 24 January 2018
 */
public class SubString03 {
  public static void test(String letters) {
    String tmp = letters.substring(9, 13);
    assert tmp.equals("teest");
  }
}
