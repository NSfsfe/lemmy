-- SPDX-FileCopyrightText: 2019-2022 2019 Felix Ableitner, <me@nutomic.com> et al.
--
-- SPDX-License-Identifier: AGPL-3.0-only


-- Rank = ScaleFactor * sign(Score) * log(1 + abs(Score)) / (Time + 2)^Gravity
create or replace function hot_rank(
  score numeric,
  published timestamp without time zone)
returns integer as $$
begin
  -- hours_diff:=EXTRACT(EPOCH FROM (timezone('utc',now()) - published))/3600
  return floor(10000*log(greatest(1,score+3)) / power(((EXTRACT(EPOCH FROM (timezone('utc',now()) - published))/3600) + 2), 1.8))::integer;
end; $$
LANGUAGE plpgsql;

drop index 
  idx_post_aggregates_hot,
  idx_post_aggregates_stickied_hot,
  idx_post_aggregates_active,
  idx_post_aggregates_stickied_active,
  idx_post_aggregates_score,
  idx_post_aggregates_stickied_score,
  idx_post_aggregates_published,
  idx_post_aggregates_stickied_published,
  idx_comment_published,
  idx_comment_aggregates_hot,
  idx_comment_aggregates_score,
  idx_user_published,
  idx_user_aggregates_comment_score,
  idx_community_published,
  idx_community_aggregates_hot,
  idx_community_aggregates_subscribers;
