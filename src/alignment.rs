use serde_json;




/*
{
  "referenceName": "11",
  "start": 5012490,
  "end": 5012566,
  "mappingQuality": 60,
  "readName": "SRR098401.47261959",
  "sequence": "AGTGGTAGACTGTTGCACAGTATCCTAAGTCTCTGAGGCTTACTTAAGACTATTCTGGCCTTTGTTTCTACCTCCA",
  "quality": "B?ABBBDCCGFEAF<I?IHECDDDIDEHGBHDIFEGGHGFDCHDDDGGCHDDGCHEGFFHCFE>FFCGAAFGBAEB",
  "cigar": "76M",
  "basesTrimmedFromStart": 0,
  "basesTrimmedFromEnd": 0,
  "readPaired": true,
  "properPair": true,
  "readMapped": true,
  "mateMapped": true,
  "failedVendorQualityChecks": false,
  "duplicateRead": false,
  "readNegativeStrand": true,
  "mateNegativeStrand": false,
  "primaryAlignment": true,
  "secondaryAlignment": false,
  "supplementaryAlignment": false,
  "mismatchingPositions": "76",
  "readGroupId": "SRR098401",
  "readGroupSampleId": "NA12878",
  "mateAlignmentStart": 5012418,
  "mateReferenceName": "11",
  "insertSize": -147,
  "readInFragment": 0,
  "attributes": "XT:A:U\tMQ:i:60\tBQ:Z:@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\tSM:i:37\tNM:i:0\tAM:i:37\tRG:Z:SRR098401\tX1:i:0\tX0:i:1"
}
*/

/*
struct Alignment {
    "referenceName": String,
    "start": u64,
    "end": u64,
    "mappingQuality": i32,
    "readName": String,
    "sequence": String,
    "quality": String,
    "cigar": String,
    "basesTrimmedFromStart": i32,
    "basesTrimmedFromEnd": i32,
    "readPaired": bool,
    "properPair": bool,
    "readMapped": bool,
    "mateMapped": bool,
    "failedVendorQualityChecks": bool,
    "duplicateRead": bool,
    "readNegativeStrand": bool,
    "mateNegativeStrand": bool,
    "primaryAlignment": bool,
    "secondaryAlignment": bool,
    "supplementaryAlignment": bool,
    "mismatchingPositions": String,
    "readGroupId": String,
    "readGroupSampleId": String,
    "mateAlignmentStart": i32,
    "mateReferenceName": String,
    "insertSize": i32,
    "readInFragment": i32,
    "attributes": String
}
*/