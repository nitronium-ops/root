use async_graphql::MergedObject;
use mutations::{AttendanceMutations, MemberMutations, ProjectMutations, StreakMutations};
use queries::{AttendanceQueries, MemberQueries, ProjectQueries, StreakQueries};

pub mod mutations;
pub mod queries;

// This is our main query or QueryRoot. It is made up of structs representing sub-queries, one for each table in the DB. The fields of a relation are exposed via the [`async_graphql::SimpleObject`] directive on the [`models`] themselves. Specific queries, such as getting a member by ID or getting the streak of a member is defined as methods of the sub-query struct. Complex queries, such as those getting related data from multiple tables like querying all members and the streaks of each member, are defined via the [`async_graphql::ComplexObject`] directive on the [`models`] and can be found in the corresponding sub-query module.
#[derive(MergedObject, Default)]
pub struct Query(
    MemberQueries,
    AttendanceQueries,
    StreakQueries,
    ProjectQueries,
);

// Mutations work the same as Queries, sub-modules for each relation in the DB. However, all methods are directly defined on these sub-module structs. But they use slightly modified versions of the [`models`], marked by the Input in the name, to get input.
#[derive(MergedObject, Default)]
pub struct Mutation(
    MemberMutations,
    AttendanceMutations,
    StreakMutations,
    ProjectMutations,
);
