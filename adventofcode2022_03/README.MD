### Advent of Code 2022

#### Day 3: Rucksack Reorganization

Language: Rust

[Problem](https://adventofcode.com/2022/day/3)

#### usage 

```bash
> cargo run
```

#### output

```
rucksacks: BzRmmzZHzVBzgVQmZLPtqqffPqWqJmPLlL
rucksacks: hpvvTDcrCjhpcrvcGGhfLHMlLtMCqflNlWPJlJ
rucksacks: hGjhncHhGnhbTHczBBZVVSbRwgSgRV
end of group, repeated type H - priority 34
rucksacks: rWVQjPQQjGRWNSrWrPjcptwBpqqJBtJBBcZgMdtq
rucksacks: zzmmpzfTCFpTlMlJJwBgMlqMBt
rucksacks: TvLszpbhhTLmsnRQPDQGWDWRvQSR
end of group, repeated type p - priority 16
rucksacks: zGzvLlGlQHLGBQZlhBWhdjRdmdWRcjPj
rucksacks: fTJNfTfNSRWPhjdjfj
rucksacks: pbsbVVnpSnbVTprnbqqrzvLLgQlGLPLHll
end of group, repeated type P - priority 42
rucksacks: ZCCCsWvNvmsCsCmZLZmgMLRpQMhwQRpQRfphfprpTfpM
rucksacks: tlncPjzlndctbzcPPBcjwDphwrfGGDffbDRpDTGG
rucksacks: cdqnddwzqjNVWVLZZLZq
end of group, repeated type w - priority 23
rucksacks: DTLbDbRrlQbwhhNrmmfwdt
rucksacks: zzMJMzjCjJJjvLjMPJpcgPpzfhHdfqWcqddwtwfqdttcNtdN
rucksacks: pJCzVpCvDZBLsVRQ
end of group, repeated type L - priority 38
rucksacks: STzBBbJzRRBZBRTqCCsfZLtNNLClCsfh
rucksacks: jsQnnQjjHcvQFrcPwCmtLCNlvDfftfff
rucksacks: sGFscMQQMMpqzqbMbd
end of group, repeated type s - priority 19
rucksacks: QlNDWGsjQjgQllWQsbtzqTJczTJcbFmmFJJP
rucksacks: MhLrhgLVndRmzJFzVqqJqP
rucksacks: pSLnMdwhwdRZRSwhLZwLhdGWQjlsgWjNQWWSvgBsWDlj
end of group, repeated type g - priority 7
rucksacks: THjSRFSddTjdBTcPLcVVvVBw
rucksacks: GzWnWfndWfznDfsnsBsPVwVwPPLL
rucksacks: zNflzJWqqzQDdSStHNZNpFFtbj
end of group, repeated type d - priority 4
rucksacks: FSzDmsFSFlDlBzqVjqHHjHHpVgHLbp
rucksacks: rTrTtTQQntRQnQJQgggHZttVgHLBLhZL
rucksacks: WTJJRRQCRRJTRdBCRdvRNDFSWFMPmDlPPSsNPSzS
end of group, repeated type B - priority 28
rucksacks: WQldlMtMVQgVMQHnDGbHGGnRnQmD
rucksacks: rqcZPrCFjmHlbGjZ
rucksacks: zSScchqwchBzTzFzhhSlcCwNtdVWWJgsVdMtWNgNVWTJTd
end of group, repeated type l - priority 12
rucksacks: lMZqjMWllrTTspjprWWSSwgWNSVNDmWGVwFwgN
rucksacks: cdCCdLHcnndHJnmCRntLBnRzDvFNtNDVzgSgwDgFNVzFVv
rucksacks: BRLcCCJCmJdcRhfjPPZphrlrPqlZ
end of group, repeated type m - priority 13
rucksacks: GdGqcrrZGDrvDJJqJHcBvmFFgmFMMgMgBtMLTssLmF
rucksacks: NbPVPDlljPmTmsTj
rucksacks: VfQDhflCCRWdcrQwJvvnJv
end of group, repeated type D - priority 30
rucksacks: RLcWgLCqqPQLcqZwzHgwmmrmmtgwTw
rucksacks: DhbhNrMpnJSDJwVTHmmTVnTTVj
rucksacks: lrsvblMDlcWcfQPQ
end of group, repeated type r - priority 18
rucksacks: PVldlphHwGwJJGdjZZWsRbbsGsNWrWQbNbQR
rucksacks: SqcDvTmDLtfmSmtqppfqzTgTQBrRQsbCFWbNNFQFBBrRbLNb
rucksacks: MgtmTgtfpqVlnVddZMwV
end of group, repeated type p - priority 16
rucksacks: BdmfmPBPSbSNdGSdvWrwcZrccZPPcZnH
rucksacks: jzzLsjsMRlQQVHwswvvZrCHrrT
rucksacks: VqhzVFzplFlpLwpMphLRQQVRmSqgbdGtNJBmNSmgGbtggSgt
end of group, repeated type w - priority 23
rucksacks: DHVpNZjdZjFZWVFHpvFvzmlRzPnlfznFRz
rucksacks: lrTBTsBwwMbrrwLPPfwGmGzvRf
rucksacks: scrtMhMCtJBBBclbHdHttWZWDSqDSjHj
end of group, repeated type l - priority 12
rucksacks: wzqsPmqsbsfqBwPMNRMMZcZmFFNtZM
rucksacks: CgCnhlvvLJgcRFNNBdCpWM
rucksacks: QnQrVgHSvVHjbjTGBbbTHb
end of group, repeated type B - priority 28
rucksacks: HdrVrdqFDdZVmHgRmDRFHMnTdTssMGnLnPJLbPTbCs
rucksacks: SczlScjwcNzplNzQSSfjwQSrTGGsbTsnTCnGTMCMLMGGbN
rucksacks: wQlfjrhfhQFHqZhRZRtD
end of group, repeated type r - priority 18
rucksacks: RsfJDGJvzPNcjpddSWJWMd
rucksacks: LLCbBCwCrCmVVnrmhQFmbVhdcdlWpjZzSpMdWSpcWczSBj
rucksacks: rrLCbTwnHTvzvNGT
end of group, repeated type z - priority 26
rucksacks: wPhPhbCqqSCrtJDlqvlrJr
rucksacks: RVVZddLFRZZcQLvJJtzptlgPJp
rucksacks: TVQRZGVncFdTGWZdCNShHhfPNwwsWPwb
end of group, repeated type P - priority 42
rucksacks: dzLVzPSgrgDDDCMSMLLPwFmdTTcsvmwNwjNsHcFF
rucksacks: nWBGntQfGNGBflWBBqlpRQGbWFvjwsbsFswbvTHjjbmHTc
rucksacks: tBNJBnGBflQnDPJrPhDgrPVg
end of group, repeated type N - priority 40
rucksacks: VtWztWtqpqzWpWzqjNRjNpWTmrrmrSbnmJwSJwnMPrCSJVwM
rucksacks: sDHsBDhBdsBZGcHvLHDLhhCSnRSwCJMZrPbmnMbJSCSR
rucksacks: ccLRhgsLBdRsdHNTFFNNgqTglqzF
end of group, repeated type R - priority 44
rucksacks: hztlmDhPhgPlPNNgmZMCbmwwQjcwjjwMjVCd
rucksacks: RSJRrRqnqQJFqvnTGrHCcHHCCHHbHHMcMvdM
rucksacks: qGJsnQTRsStsftPlhPNl
end of group, repeated type Q - priority 43
rucksacks: BFFBLPRCwsLwhlPlRmhcGGrbmmGjfNTTnp
rucksacks: VJMVpzgqggJnrjmjNcMjmT
rucksacks: VqdSZtQgZvtdzqHqHtVZdVQpCDWWFdwlRPDpWPPBCswlWD
end of group, repeated type p - priority 16
rucksacks: fCWCsjPzcbzwRSzVTzhhDLqvdg
rucksacks: TmJtrNJrBLSLJqgS
rucksacks: ptNTQFHrZlnpFPwsWMbRjCpcjR
end of group, repeated type T - priority 46
rucksacks: nJmQNCmbmlllmbClbfMLjMFqbGBsdLFq
rucksacks: ZcgTWcTnMqqMTBqF
rucksacks: tPgctSnPctZZgDWzZgQHwNmHlhlmzlQhlJlw
end of group, repeated type n - priority 14
rucksacks: ZpTCwpffdslvgShCBhqhRz
rucksacks: FDMPnNFNmBPzvRPRBg
rucksacks: nNgMrnnDGjDmJMmnFdZTTsdsrZrslcwcQr
end of group, repeated type g - priority 7
rucksacks: pTmczpCldcdDDnPttpvWSqbpJf
rucksacks: jgjRZMGHhGLgQrjvPWzPJgJvzStbbq
rucksacks: LGNLLNBBzcDFCBwwFC
end of group, repeated type z - priority 26
rucksacks: nJTTqnrNvTzNMzzNfqrTPrJnwpwPpZpsHccZVsBRpcVHwpcp
rucksacks: bgDhgbghLWmFmStctVpZtBCVCCpfZp
rucksacks: LLSgLGSjggFGbSSbmMnrvqvzjfzTNrJrqM
end of group, repeated type f - priority 6
rucksacks: RRpDmmPMTjwfGmJQgQ
rucksacks: WsNscdnvvdVZFVnnrZbjjflwljlbzfGFjQjq
rucksacks: NnLZsNnrrVVVcvdBLTPCPCRMwhPMBMPhCt
end of group, repeated type w - priority 23
rucksacks: lbVvzngGJnVbJHpHtHNPpdSQvc
rucksacks: TsMBswFZsWMWBZMNwPtNNtRNHcNpSQ
rucksacks: CcZCTrZDsjZTsTsshWhrWrTnfgbLDfJzVVLVVlgfnzfVGV
end of group, repeated type c - priority 3
rucksacks: JzTTRtJRZWmWjrMHCT
rucksacks: DDFGlLGcGlSSSLsFGBspPBmNMBHMghmWNmWjWCmWtH
rucksacks: nSSpnbsGlLDnpPsSSspFtVvffRQdVzqvvbqdfVQwRz
end of group, repeated type t - priority 20
rucksacks: sMhzszlHHDsWbthHDqsbJjpLNtmjVJmVLLVLVLBp
rucksacks: nrTPrGwfPLdprzJzdL
rucksacks: wgPQcTGGzgccwCgnRwgRChFhlWSDqWWQMWhssSsMQl
end of group, repeated type z - priority 26
rucksacks: NSNmwtpSpCpvMphCsr
rucksacks: PHcRGPLJMrsvzsqG
rucksacks: QHjbnRMcfbPbQZmlZgZlgBBQ
end of group, repeated type M - priority 39
rucksacks: cPRPbhQjbQRdtPQdLqLHqzFZjCFCqLjC
rucksacks: mmfsnnwrfvwrfSNZFzHHLDCFNlLlqDlN
rucksacks: wsmrwswwGTffMrBnmQttJtcMZQQtPJPbZc
end of group, repeated type Z - priority 52
rucksacks: MvBPDDRRdnnvHPCHZLHZsFLL
rucksacks: rmJcbVqbcjWwWjQHLzTZFTHSzFrpsz
rucksacks: cmwllVqqGJbVVVmmqbQcmgRnRvGhGfgDRDZBBBvRdd
end of group, repeated type Z - priority 52
rucksacks: nMvMhMnvhnbTZWSSZgHmGJDFmmNDzBmbNmdGBN
rucksacks: rCsPLRCssRjrLLsrLlwRVrcNJQfDQfdBmmfNBGJNzmDPfB
rucksacks: CRjCpLltgtJgJJWq
end of group, repeated type J - priority 36
rucksacks: jshCzJpjzTPpmCWvSlpfwHfSWglf
rucksacks: LQMMNMnHtDtLVRvwwgRWlldgWD
rucksacks: qHVrQNHVMFQtrrBBQMBcrrZsZbzCZhbbJZJsmmsmFPTC
end of group, repeated type H - priority 34
rucksacks: JZQZnsQNMqTngZqJBVfBfPPVBNrwvfPw
rucksacks: SSmDstFjpDpCszDjcLLhrPVlGlrGGVBwrvwVPt
rucksacks: FSssFcLjFjbmFFCzjLcFLRDnMJnTHRnZZTdWqZZWnMnRnZ
end of group, repeated type s - priority 19
rucksacks: GbHRHpldwGMpWhHpCMBlCbRdVSLhnqJLSrDPLPPLPDqVDrhh
rucksacks: gvjWWQvgZFtQFFNqLnVnDnSJzzztDD
rucksacks: ZccccfTsffHdWWdRWwsw
end of group, repeated type W - priority 49
rucksacks: ClCtbHMlnnPPlszV
rucksacks: gSDWSLgWQWQJJNWqgtQjPsnfcdVcLVdVdzfzVzff
rucksacks: WQgqtFQgDgQSFqJhqhSJvNDRrZMZHwHMCbZhTpZbGHMTMG
end of group, repeated type t - priority 20
rucksacks: pZJZlCQtHFhPfdNfCh
rucksacks: zcmLSVczwcMcLDNFHdLPhPWH
rucksacks: szvVVnBmnTGQtHTQ
end of group, repeated type H - priority 34
rucksacks: RVVCNDlNGzlGZqHGHWqWhGqQwH
rucksacks: ZFLFTmpLvvmSqsbb
rucksacks: TrfpBfJpJMlnnNfNZD
end of group, repeated type Z - priority 52
rucksacks: qHHlDClHhltMqQsHDhHslGznwdTnzzwDGSdfnwGnwG
rucksacks: mZRNcNcLLPNPBFFbbPmLmbZFSCVfJJTVndVfSwnRzznfTwCS
rucksacks: CcCWFbbBLCWtgWgHjghqvv
end of group, repeated type C - priority 29
rucksacks: TjbzlnlFmfqCFFVVCRWr
rucksacks: PhMcLpPDtMLpwPDvLPJbMhSgVCGqggVqQgCqCgCgSWvv
rucksacks: btbZbNZhJDJJhDtwtsTTTmBzzBBmlNlmHj
end of group, repeated type b - priority 2
rucksacks: FqhjWtqlqmmsnFPTCvMCQMTTCjQd
rucksacks: pfffRfLpgrgGgzrNVzzpGVzRCdMCPJbwwcVMbQPCJVMVdbww
rucksacks: DGGDZRGrHggzSsFQnnWShmtH
end of group, repeated type Q - priority 43
rucksacks: vtHVVMMrvVMVrSHvLgvlHcZFCnRCZcccZtRRZfJFCJ
rucksacks: rdDjGsdTQDcNZfdncCRR
rucksacks: rBDsTwBbjbmbbQswswPhqVmmSvpVhlvvqMhHhh
end of group, repeated type r - priority 18
rucksacks: vGBLrqMNvqSLBvvrNbllLHfwStWWtFttccjtRtjtcj
rucksacks: MhCDJmhMDzmcRRcjzWfztH
rucksacks: ZQDmDhVVCQbBVdVNMvvv
end of group, repeated type M - priority 39
rucksacks: ptCtCzhWPWptnhVzzpGZbZTjTjVjFGjVFgVl
rucksacks: fQswRRffmRqZlgrqqFjjSgGg
rucksacks: HwsQDNNsDsmRLLHmffsfvHptBnhtzCvhWpZWBdhnMdCh
end of group, repeated type Z - priority 52
rucksacks: RlHzzTqczBPfbnvcpB
rucksacks: wVtNwpSZstppwwMsZhsdnLvnbtBBmbnLFFdnmF
rucksacks: WNQJMVWsZWwGJWhhSNrQzlgHrDCgQRHpCHrl
end of group, repeated type p - priority 16
rucksacks: RrZWpJZRrZpdTGstlchLGGlLMd
rucksacks: NqjDPCQPnQCSvtMzSLhhjM
rucksacks: nQVQDDDDfwBwNCVCNVFNpWpgJgrRTmLTmTmgRTWF
end of group, repeated type L - priority 38
rucksacks: SHMcrMHpcjGcjSrMMbvSvvSvwFTLJwJNtFGFWJNtDLFTLfWN
rucksacks: zqRnPfzQCRzqsmRPzznhszzLtLwQwwFTgWWLDLgWFTwTNQ
rucksacks: qVPZmRZhsCZPhZlRCqRRRCbfpccMBjvMVjdHjjMjSvdf
end of group, repeated type f - priority 6
rucksacks: VVQdHwBZLVltlddtBczhrzvGcWWFRwgsFG
rucksacks: TDTTTqqTSSqjqnmTmPqPPmTmGhRszvsrzsjRsccgzrRzgWGF
rucksacks: DpJPqpWqHbZpllpt
end of group, repeated type W - priority 49
rucksacks: cCSCFsnnZFnscDtNdJFJtJtdmb
rucksacks: VgBqBsqRrHtNdzmNrt
rucksacks: BGLLVVjRBsqPBfsGwPsMfSSZCSfTZTZQpSphfS
end of group, repeated type s - priority 19
rucksacks: plCHCHlgglHHGpNbtngNrDvBDpfQDBQfZDfWZVrr
rucksacks: mTmMLhRfwhsLPQvQZDMZQBQWMB
rucksacks: cwsssmqRTFFfFgtbCtGl
end of group, repeated type f - priority 6
rucksacks: LQPPrCPnMZwqtRMn
rucksacks: cWTSlJWlcplJdDTdGdpDlGcGgqmtwwZtqRrNRRmRdNZqmgNq
rucksacks: GSJcJSjsjTpsvWGWBHLLvVVBBBrFrzVz
end of group, repeated type r - priority 18
rucksacks: NVPCSPMNDSNFVSWCsJJJmpGmZZGLLcpZLHGGtsHt
rucksacks: fwzlBBqghqvzqqlDrHbpHjZHmGZbLZrHLb
rucksacks: dnBgnDqQvwRnSnnFMFMP
end of group, repeated type D - priority 30
rucksacks: BCbPsFFwCRHmDSBmWnvDDj
rucksacks: phhZVzdpVfQZphhZpRhSVnjmrcvvnrWtDrvWDS
rucksacks: TfQJMfLphMhJdfdzpQJRTPbwHHNlgbGwsTGgCP
end of group, repeated type R - priority 44
rucksacks: ttWLlnnvnNnBBtlTqWlpvpndQdZsQQFssFDdsRFdVdRNFQ
rucksacks: jSgrScrbGZSGrrCGsFVMssFsPPFcDDMV
rucksacks: bzSmJbfCZCbzLwllflwqtvvw
end of group, repeated type Z - priority 52
rucksacks: zmFTJwFLPmzLztmjDzTJwfNrdFNrFppBSNRGNGdbrpBR
rucksacks: gqlhWQgsZMsvqMlMMvsvqsNlLbcdppbrRpdbbcSrrbbr
rucksacks: vssCgVgCsggZQZCgsnsqWgWvfJPDLwffwTPPmzTnjTPmPmwJ
end of group, repeated type L - priority 38
rucksacks: SpcRTPQLBLWpNNzjmmwwwRrR
rucksacks: tGlfvGhfnbDlbqlChnfFMrwsmwNssTMHMHjFwv
rucksacks: ZlhtCtffCdWcZWZVVT
end of group, repeated type T - priority 46
rucksacks: jTTCcWHWJNgCGTzTmnzrmnGn
rucksacks: BwRRbFvtvvQmJJFMpMJr
rucksacks: ZBBwLvqbBZsRsbVsZSqbcZdJjHHjhfPCJfJfHhgc
end of group, repeated type J - priority 36
rucksacks: VrnDSvvrLrfTdTLGfdRp
rucksacks: zcJzmcFcHGfdGmWTVd
rucksacks: tHsMhwPVctccHFHFcbSDbbPjnNbBnbvBQB
end of group, repeated type V - priority 48
rucksacks: QttWQwLTnLnWTtnffnLQSBFVjNvBjBFNgMdCsVWsjv
rucksacks: pDqcmmRPHqgVBddjvN
rucksacks: DcclzbcbPbJLnNTfnw
end of group, repeated type N - priority 40
rucksacks: plRcpsZDGlGZvWvMCNcLtttq
rucksacks: SrfrwSjSVrSjwbmSrHzmHJCQQPQzqttNNQJMzJtqMW
rucksacks: wSHVnfHfWwwHWFVfSnfgmmRsslFZZDBBGZsZsDTdGRTp
end of group, repeated type W - priority 49
rucksacks: qSFQSgQNgQBrBHHcrW
rucksacks: VTmjVJLTwlTmwTVmsMJMVlJmPvcbvvbCBbGBPjGvBbBGWcbb
rucksacks: DnJTZwmnZRhnpqNdWt
end of group, repeated type W - priority 49
rucksacks: dTVHjZLLZDVCfVHtLDDjQbscjWbSJMJPjsbWWb
rucksacks: FnqrnmzzFllmsWwtsFtQMMFc
rucksacks: lmqzzzngGmlNNBqGllzlBNRvptHHpTCHpDLpgDZdgvHvDD
end of group, repeated type t - priority 20
rucksacks: sdRZQbCfZTSTdlfTZCffccWPHPPcPPwLwctRnLWn
rucksacks: BBJDzFVgCDrCJrqDJJhqJVVMLPHwcctFwcWHHGLcwGwGHnWc
rucksacks: ghpJgqqjCZbQdZpd
end of group, repeated type C - priority 29
rucksacks: tbcpzbHSszcHBgqHGZgJJJhhww
rucksacks: jfvdvRTffQQrrFCRFTnGwJRqNRZVpJGZLZggLh
rucksacks: nQTjTnMndlTdQFMvnrClCnpzmzDtbbmBbcPSzzlmmtzP
end of group, repeated type p - priority 16
rucksacks: BqBqTCSTcqHsJHHM
rucksacks: WWPGVPLtzVgWtjWPGzVjzVGcbDhPsRbDcsbJwNRswRDRss
rucksacks: VQfWjfLFGWLjdFfVzTZZpJTpnmlTrSQlBl
end of group, repeated type J - priority 36
rucksacks: jLNsZjqSHCsGdsmpsm
rucksacks: MvnVFzWMwMVWzfnVDwfBMfnnrCtdtPmPlRrdrJCJrtPDrrPD
rucksacks: zznfFWwMfMfFMwVTMQFnQjhjgjSZhCNbLSTcHHgbbC
end of group, repeated type C - priority 29
rucksacks: GGtssttVmvnnGNMQrrVzgwVrCWMz
rucksacks: FdhfhhcCDhHLfzclZMcrwcQMZM
rucksacks: HHqqCBhHSSpdmjGqmGjtjtjj
end of group, repeated type C - priority 29
rucksacks: bbQLtGMQQtQRQtrDtGprrrbCqwplZhhqSqmdwvdzqqqhSmpS
rucksacks: FsJjJBfnsJcFcFfjVPjWBzldqhqnlZZZzzhmnSvSnm
rucksacks: JPcFfFWjFHJVVsVjPVscsDlLNRHGDbLRMRCDNrCGbG
end of group, repeated type l - priority 12
rucksacks: JdMdlMRJnTwdvcjv
rucksacks: CDLHbNSzzLFgHvnTjrswBNBTNT
rucksacks: QgbvzSFQmZQPQQRW
end of group, repeated type v - priority 22
rucksacks: NTBrNzrpjjjCwGbB
rucksacks: FRbQlcvFvcRQQlRsMlRRRZjwCqMwjmjwJZdLJmjCZC
rucksacks: cVPPQcvlWDNhrbPz
end of group, repeated type b - priority 2
rucksacks: VdbVtbbZJdtJVVdDVZmTLqqTSQvNLjjDShhvSG
rucksacks: zplpnBnFpnrrlghGNpLNqHvqvjNj
rucksacks: cWncllnlPFWzcMwtWWtsVLVRmJWCds
end of group, repeated type L - priority 38
rucksacks: ShLSTnZnTSttTSbLQdfSZTMwcDHwwcHnJvDHnlnlclMM
rucksacks: NmPMsssRrVwjDclHJwwR
rucksacks: gNNMWGzNmqGdtfZTbGGb
end of group, repeated type M - priority 39
rucksacks: sWNNlRHnmJtmntJt
rucksacks: brbbBTbbFbCbqqGgBTrCfmQVVZfSSQQSVtJZSrVZ
rucksacks: bbFqvbDvvGGLGbCCtBGDLbLlcPNHhhccPNcdPPchlsdR
end of group, repeated type t - priority 20
rucksacks: DCFvDvnCnNfMBmMMslDZML
rucksacks: SQQQJHwpSgJSJHQWSWHqJWWbmcBBBLLTsmhhTcZbMhmlshcb
rucksacks: RJRgpJHssgwSQHRqsQPGGjjtNCrrFvvnFjjPrP
end of group, repeated type s - priority 19
rucksacks: mThmsgjzTPjMpcvtWP
rucksacks: GNNBVqVGNZbbNbNqqZQVNVNbWcpdtMCcpCtMWCdCPpQccmpp
rucksacks: VSmNrmmbBfZVlsrssrLTRhRhTn
end of group, repeated type m - priority 13
rucksacks: TdmCvLDCpTRNTdFbbWnnSWCfhjbbzn
rucksacks: GrrMsPVGcQHBGMbhjjSgWfHHDbjb
rucksacks: BPBVqqrQPsQqwrrmmmJdRLDDqFRplT
end of group, repeated type D - priority 30
rucksacks: fpDDJljDlCfDTjprjrfbddWthCSCtdPPQFhSSSWW
rucksacks: HsLZgMGbgBBsNzMvGbdVtVQzFRQSthhFPdtP
rucksacks: sMBmGBmbNvLHGMnrDppTcJmcjpqljf
end of group, repeated type b - priority 2
rucksacks: ptSpSJQqpbNGGDDhcMWrlNHcZZWWls
rucksacks: zRLRRRjvvgjHMMsMpWpc
rucksacks: vmCPLCgwvwdnCzmvLbpTbVQqJJPbJPpTVq
end of group, repeated type p - priority 16
rucksacks: TJCfhhJVFffrJJQQllNWcvWhwvWD
rucksacks: GPSGjjpLslBbpLpLqqqPDvdwvwvNzQWGzDDNdzGN
rucksacks: msbRjbpPqsRpHnlZrmJlnVHT
end of group, repeated type l - priority 12
rucksacks: GGfFsCCTvGDsfTTrhsCMMzptZJMdpdgtrpdMcV
rucksacks: LBlwBHPSqjwwlVggHpnMZcVHMt
rucksacks: ZlZZlBbRPGGTGfmRsD
end of group, repeated type Z - priority 52
rucksacks: CtCjbVvzQQZTWVdd
rucksacks: MlSqWlmsmGBSHJHTDFHZ
rucksacks: pcqsmsplwsqclwRtRWgtRnPPvb
end of group, repeated type W - priority 49
rucksacks: zCrzCrsdjrhGDCFqGDjRRPtpWfQQcpfQZcCZPp
rucksacks: VSVwVMgLHHLTwMDTMMVnbWPRZQRcRQPptWnpbZcb
rucksacks: MNBBBlSMvLVwTlVTFdNdhNhFsqsGDrzm
end of group, repeated type D - priority 30
rucksacks: rBLWTwTThWwVVDTwHBsZZWppvpGtpptppmRvFFFMFMfL
rucksacks: qPPNCCbqcbcNqbqQjjJQqzjRpptmlpMGmMlJtftmtFHpMt
rucksacks: QnCgzzQbbQqPcPQnncbdQdnVTwDssZgrShBTVgZZsBSDHT
end of group, repeated type H - priority 34
rucksacks: PFGJFqnfqmPgFJQPWdbLdpDRhbphWjDm
rucksacks: rclNHvcrzCNwrWRprjdMMMph
rucksacks: wsZHwZNvRRQsQqBV
end of group, repeated type R - priority 44
rucksacks: LqlGCPlPLTCPqqQlpqLlWfBfWgcHNRJRfWNsncGH
rucksacks: VVtdwVtDDdVmhrdwSBmjbdzNHgfgJnNnsSnHsNffHgRsgR
rucksacks: wVzhbjmDbDrwjdbztFDDthMCvqPppZQBQLZQTqTvFTvZ
end of group, repeated type B - priority 28
rucksacks: BnQnQFwRmRwmwdBSFDFnmSDVLCJTCTppVVmGLVTCLcgVpC
rucksacks: ZlWvhvZjNrbNvqjNhlfPfqjCGHrsspggTpVLpsJCpcJVgg
rucksacks: vPzNvqjWhqFzGSnRGMDG
end of group, repeated type G - priority 33
rucksacks: wZnMZzzZZchDRtVsqtCtwV
rucksacks: WmWpWWmPPWrmrmBmWrTlTFPNVqVCRSDCQHcqVTtTqsSDSTSD
rucksacks: PrppdFlWWlfrWmpWFffrdcGjJJGggnnhZGdLLgGGndvz
end of group, repeated type c - priority 3
rucksacks: FShHNmNhRhNJmBnQBQJrmP
rucksacks: VTgzDTjwfffwzDvwlcczzVSJbQlBQSWBWCnPJPbJWWbC
rucksacks: tzSVtzvSvGSRZqqFMNtpRR
end of group, repeated type S - priority 45
rucksacks: hPZhGDZpnCGtDhznjmLmdJffdNzJ
rucksacks: glwsSrQwBvLdgLzdcj
rucksacks: QsRbHllzzlHwHlBszWlTBFbpDPMhbPDVGpGFpPtFPp
end of group, repeated type z - priority 26
rucksacks: SRjStRDctgDSBzLvPvNrDhmPLr
rucksacks: QqTHGTPJmmHmhNmH
rucksacks: TGQZsTqFnQZCJTPsnJnZQMjVRBVtcVRSVRBlwccSCtBS
end of group, repeated type P - priority 42
rucksacks: bbsNsvsvnNPTRRllbblLqhtQCqQSLCGGHSqHNC
rucksacks: wFpzFgqVzqVJWFDwqJDmSBBmHBHhShLQhCGSBCGH
rucksacks: MJVpFMqgwMqRRbZsMbZMrP
end of group, repeated type q - priority 17
rucksacks: PPdDhvNDQdmgQPZmQVHHtHGGWVGbffWGvs
rucksacks: MMLCTRRLlLclTLRMRLCwMLHWVctbVVHWWWFfVjVGsFWW
rucksacks: MRSMMlpTJRqClBCRqBDnzqgQPnqgznZPZqbP
end of group, repeated type b - priority 2
rucksacks: MrMNPNNpjvdprWtrpMsthqBfqlnfqcGhVBqFRcnqFG
rucksacks: QbDgSSQbgSDDmDVmlqSCRllRcFqnqfBl
rucksacks: QVJbVmwwDQbzVTgbppNJNMWNjNNPrdpM
end of group, repeated type V - priority 48
rucksacks: WwJJNbtHfpLpVgZZPVFhZh
rucksacks: vmmqlDvRvRfqBSrlzmmMjRBhcVhQVZhVghCQQQQTcTrPTP
rucksacks: jSqMmqRzMDDjvqlBqsBMBmmwGNJwJnwLNfbGwddswnJtJH
end of group, repeated type f - priority 6
rucksacks: RLgRmRggbvbzzPmmRNmzsQWFtSGNtwSNQnntFwnnCw
rucksacks: pDBrBHpHhlldphHBHhJVFSLnWWFJttCtQSttSS
rucksacks: hfHrpphHBppfTvmzgMmbLbgf
end of group, repeated type L - priority 38
Summ of all priorities: 2798
```