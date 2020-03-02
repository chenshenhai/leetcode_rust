/*
题目:  最后一个单词的长度
给定一个仅包含大小写字母和空格 ' ' 的字符串 s，返回其最后一个单词的长度。

如果字符串从左向右滚动显示，那么最后一个单词就是最后出现的单词。

如果不存在最后一个单词，请返回 0 。

说明：一个单词是指仅由字母组成、不包含任何空格的 最大子字符串。

示例:

输入: "Hello World"
输出: 5

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/length-of-last-word
*/

struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut result = 0;
        let str_vec: Vec<&str> = s.split(" ").collect();
        
        for i in 0..str_vec.len() as usize {
            let code_vec: Vec<u8> = str_vec[str_vec.len() - i - 1].as_bytes().to_vec();
            result = code_vec.len();
            if code_vec.len() == 0 || (code_vec.len() == 1 && code_vec[0] == 32) {
                continue;
            }
            for j in 0..code_vec.len() as usize {
                if code_vec[j] < 65 || code_vec[j] > 122 {
                    result = 0;
                    break;
                }
            }
            break;
        }
        result as i32
    }
}

fn main() {
    let result = Solution::length_of_last_word("hello world".to_string());
    let expect = 5;
    assert_eq!(result, expect);

    let result = Solution::length_of_last_word("a ".to_string());
    let expect = 1;
    assert_eq!(result, expect);

    let result = Solution::length_of_last_word("  HrGChmcHBfJgNAkvUWiKpGMIXKfgWpaCayzCIiyLIiohwtlUTyBkJhLIpkGQtCEuPuvcxPeESbjFAKehZlgABMSdhnSXHFmpoVLgowTQWueRWDzzDtuUPQchqQuKgDivYcqdRSDVdSXuSLbNXVOXYBqaKxXMQrLuwYqTMttANModczDYInVs NiBMMrIoJCUHOpnvOSCEhgubNpRqDPyvreVkiwmiEIVgdTJqevEGZrEOwfySiHCVtKapwpKleAGHGkotZLXbLWXdDg         AEWrFwRTOmDXOpJkdxMqgjHunltzkfIHipwouEeaHJxMZGnyWTVxhUAgJnbrxOpheFLteIlrggIOzqraBWkghyFRVxLyCGnFCFcBtxqFiRXidCundsPyuUiennOnPJBKToAtMlsnOuyVDzfNNkqNvzOxoGDCiMMogqeDBpqyhVisdIlsXrNctqJPuAwYFqICaFKVGnLhHCEkQplQzLSowPHrKWvAJNKNAuiTQslPGLUQcFKysHijmyPUdzThxngUiDdrsPjesNVLLdxwZEkiMkysoBRwZfNaveBIHPtMOUMZLmknTMgNsyLxBJMdqYGKFXWfqBReSKzGhVUOIRbuAwYosZQNOmSmrJaLdXmuVFPTTttpfyvoAtoBepNtPhkgGCjlKxjiLHkjOjPQYbktsTVPdhAqjRsvCDJjrhItWetpxcQKWImOYSuvoRVpmJTyJz           BLHHzdjwSSqAfAhUbDmNNetxPtFXdRbUsOoTYACIRzVjamPhIPbCKudgGUhCIKXJPgltmKTmbcQqxZUeJduunAZjZK     pDobjfINKKkAMIqDWyiHlhPyZnUgrjupuFOxAxCTgnykmYhBdhUvknnqPxCffZTrirecgEwRDAZvltwgkkogERipXSNNwGCTzqsSEnLmuGqUoxnUFBWFPqxiRbJAwQSYYZuzAPWpMboYavDuXYbFpZVeJHILWgEdyWADAEXJqfYozxYhdAjT        ljqOcqkMxKlmUTKPNragwxQTYVvZEfAmVxMBctmhuyKhshkkCCnaGhgfsSdXpsAfoURQpSPADotGrJfyAUOrJcjiwz               BgWbHOvqhyCoakGFWUbpZRfJjArycRgsoxKcsGDMzaeKmrcXzikwiaWzClboCHsxaETnGQfxURJmPRacxnIYsTjjDAMAVfMlxeeVOEgMHuEBHmsQfSsHkqVaHAUMOxaaNTGPPKmtJgZhWywgRQeTPIRWXkAOavnKegKEIcsfEOlCRLcNYUxj       JLaIVUGXUPMExJjrjjTAmnmnorrJzuuEPQiApqCnitcoYNwXjAIQtdOQXZPCYFJntNsCMLyBmxQqtNdysaryMIkIkbrdgtiCsMnIbKFxJZrSPKCsdtzQWqiROXYFEKFfMbcpsEUyrpNendVeLVXprvUXRCNbYwOdHoLzaupFIOrZVLZnoPHe  dtjvAoEYUHHNVnGbAwYGgzfDAlhjwhwohhFGLZhsQRfeZgWwKLTFyxLRWdLeyjkAlMeKxMyQpXfoQaHxnYCVsMOnaI       NZIpbpTkwAwpUYWCXYLNpMzbDHPnnRdXyhJYAlKwnRCtUbHZXqdjCGJdKgbBLUgZFLIQuZDeBznBiSsRLiBFABNRuu             IStumRazQODyumCmlMVtNgFVFMpghbAVuVqzBVnddEDhevVvaigXPrkyllOzcZNrOHJGxesciVhqTkOyBalmguWhlctWQBdvEwWTDnbpWiTeLfmALiFAYnIjqYxvmYEuXafNRhFhHHmOgbnBbOEoEWLhncTcHLfgqpysBoWLmnGFXjfKOriT            afkCxpauSFWJjBENCvkMJlUXHwXTwNyhvrBOLYgXZTyjjyzokZnDoGbKyBruGNBZJbpwIAIlPfyjVoyjQFtNEOZyoX         yfaCoBFuyIsOaXUHjgvAIEXHCKzmDCQVaqPhTMmOjmynkFaEfaoTWAYbQvquzfbEicpGaxymoESHBtakgghkWLhMhF  uBxQLvUgpnzATEkmrWsxLzYMHQwuBpXtvhevZxZPMRXCopYfAjTWbTImPGvoCYoYsVQfrxjXsNNYhuEncJfooYYAAEfbChMBOEmvDwuDTEdEEEtRFOZQfhdlFXxRRxfvlDurickNHbqlQvykEvkcmiwPInjjIzyrafdGOQmTywGMlwaOdOda       hzyGKcCnaPpmBBKVRtWMpHFrHxyDCEwHYibEojGciOLkBCQQkmVEFEQdgjOgLqUuetzLpsWHtVLambwOEfHkgfBHEG           PXgozcxuKLgFMUcBDLXHtcYcbcFZSNbPqRClkRnTJYfHTUmAmTRnvkbZDbxHkOncHSDDnIVeiIUAvaflbpleISZfDOqbFMXnSOyUJMELJAnHnGxPMCxxwOgUiuVSywRNTQlYBiNzYVYInuuLWHRPwZDHWaRtpMWAzCsKsAMCLgnIrpTIamdj    UqhCFRTlxbZPYkRsQjnrdjSjyXSLjZRODjoKLSnyrNOcxBvYrPKZvEvadyosdCkxmhpyyzIKpgkuToBFBPxfStFRND                   MvvnPuHVnSqLWgVWXmSAqpLnzIgZRcpAVkNzEcASysWKBePgscCLuXhWSokdPGptGRaqtrfKGnbFcwcDMrYjywUdluBoAslCuGdaZzLUXpKQPblKzKvJtEmjhlFEmXNFcqdSkcZJmbZoNLuoIxGJqPBzYgiljypFTlTsKCMAgkhosYCHJzSQtQrGnLLEcCWrzlmWHIqcLPRXGeAVTtyREFoaXAklavpBBtLemmwfFmEIpIAQhDxmYPpNBMZneXgIAjalxSyLJiwAuhRSAhhXziZSRyekhwFgkgVcQeWrAGplHYzsSCdlTMNGSthjnAnZwyKWAuOErSeOKKmInlpDViDulvSNcCicsWYAzfwZuIOmTwFyqVKRuTsOeyMiMlAQsIcZuwXfIYNCpXyIAvtboFKpoPWfhGqxJvruWTkaxdmKObizVYtrUzwguSwbuRdylm          ysqClkGhTOhcUdXnGXWVZOOTATfRlLriHWjpbuDAySrIePVVeKNaIiJTEPXCWZOwpkIuggNApptPkhmVoHrRFOAnIdvgBACxNAoXNqFMngitXUXKpcJvsCuiGrgDifpucMxMUpgApGgaIZXIqoFhRrWQPjIOrShsYrJewDrGsDdthMpQNShc               ybuXwsWGQOkPcabMIfjEFaanbAGRnPnQRDCmiMtACkttvxnJMKXbLQQcvesVjaJeXooBeHkQZVwimhSVDNZTGclzRFPeDSyVzjkwuYQGQdSGKpKtDzSXMEGqjHEbOcfLzvIgszzbllwjFNgUNYLSVEjCNpcbLtRAEajvMZGssKjEbPNYSnVG              gMPkAmqEgZoHvlPLQWDIfiKfhBwyOVKaGKIPRkOyxUJnFeSsfMAEDXjQwbGgAeUXeRpxpmrgSUynqxTNIdEsXvlCLe       LLCgDaojcmUBfxZcnvwJsIsOzvcwMSRwMZVnWXrKrQGEgevfVuqJhcvsWKyWnecSrswwMDrZGRalqADvhETIgQINAv                  vYGQXSehuBmBYEgcLzDWQHdmDJWpPTvBYaTGOHBQiWNCiESmiWmnqRSAKiJfHwQUycukpGMkczEgOpnHBXZKmywydP                VzbVcrvJJxWUVUgfTNwciYongJgXeoSqDhQZHjIxjSowMiguEKPymXfKEluPtNwLkzKZiSuhguBljwHkgVnOWlkumv  KftQJtqddKVlNgprWGoxXfYSmvbziAqzrGGeYQUdPtgkYngeWErfWQiDTrpRPfuVwuOHxHVghScPRpfEpTAqEEHpjq         FBzJPecNeRSvTRURYyFEiFqglFBMkitIVqPxFlWSSMeQhxzTNAfGAIlgCzxzqgdRkWqfwPSOteIXbsfWeKZNGibYNO              XpqDNmDSlvBsVGEVdwHKjxkFQsCUsHSdRSyXdwMMhXqKuJylNplwleAbVvTAzEAoYBSBUAyFDLiMWGlmYWnVFFAaNDsVOWfKFgxLwwtYjfDoTcVurgtnKZPKvBRSdnjAlqaXPQygrVyZKdYXLSvZsSSnHyXMNESZRTmaMwsTTIqcjrKkLraDBbqhMAdrzwgDqPGSCTzdCVLrKDofmpzCEXYSgHbFMeUHfBgIASiCAjjpCAefHqLcFOifakIjvItLxSbwRzCkWoyQjy".to_string());
    let expect = 270;
    assert_eq!(result, expect);

    println!("success!");
}
