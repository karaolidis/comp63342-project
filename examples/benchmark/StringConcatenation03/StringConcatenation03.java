public class StringConcatenation03 {

    public static void test(String s1, String s2) {
        System.out.printf("Result of s1.concat(s2) = %s\n", s1.concat(s2));
        String tmp = s1.concat(s2);
        assert tmp.equals("Happy at Diffblue");

        tmp = s1;
        System.out.printf("s1 after concatenation = %s\n", s1);
        assert tmp.equals("Happy at");
    }
}
