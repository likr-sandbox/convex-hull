const mod = import("convex-hull");

export const convexHull = async (points) => {
  const { convexHull } = await mod;
  return convexHull(points);
};
