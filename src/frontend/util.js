export default function aspectToWH(aspectRatio, isVertical, unit) {
  var splitRatio = aspectRatio.split(":");
  var n = splitRatio[0];
  var d = splitRatio[1];
  return {
    width: isVertical ? unit : (unit * n) / d,
    height: isVertical ? (unit * n) / d : unit
  };
}
