
std::thread BoneThread[128];
int boneThreadHandle = -1;

float distance(RE::NiAVObject* actorNode, RE::NiAVObject* playerNode, Actor* player, Actor* actor) {
    return sqrtf(powf((actorNode->world.translate.x - playerNode->world.translate.x), 2) +
                 powf((actorNode->world.translate.y - playerNode->world.translate.y), 2) +
                 powf((actorNode->world.translate.z - playerNode->world.translate.z), 2));
}

void logPosition(RE::NiAVObject* actorNode, RE::NiAVObject* playerNode, Actor* player, Actor* actor) {
    auto playerBodyPartX = playerNode->world.translate.x;
    auto playerBodyPartY = playerNode->world.translate.y;
    auto playerBodyPartZ = playerNode->world.translate.z;
    lb_log_info( std::format("player.BodyPart, {}, {}, {}", playerBodyPartX, playerBodyPartY, playerBodyPartZ) );

    auto actorBodyPartX = actorNode->world.translate.x;
    auto actorBodyPartY = actorNode->world.translate.y;
    auto actorBodyPartZ = actorNode->world.translate.z;
    lb_log_info( std::format("actor.BodyPart, {}, {}, {}", actorBodyPartX, actorBodyPartY, actorBodyPartZ) );

    lb_log_info( 
        std::format("diff, x-diff: {}, y-diff: {}, z-diff: {}", 
        playerBodyPartX - actorBodyPartX, 
        playerBodyPartY - actorBodyPartY,
        playerBodyPartZ - actorBodyPartZ ));
}

/*

// get base formID (without mod index)
static inline UInt32 GetBaseFormID(UInt32 formId)
{
	return formId & 0x00FFFFFF;
}

*/

struct timings {
    int last_ms_t;
    int last_ms_t_1;
    int last_ms_t_2;
};

int CalculateDurationMS(timings &ts, std::chrono::time_point<std::chrono::system_clock> &last_ts) {
    int elapsed_ms = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now() - last_ts).count();
    if (elapsed_ms < 175)
    {
        elapsed_ms = 175;
    }
    last_ts = std::chrono::system_clock::now();
    ts.last_ms_t_2 = ts.last_ms_t_1;
    ts.last_ms_t_1 = ts.last_ms_t;
    ts.last_ms_t = elapsed_ms; // average of last 3 strokes

    return (int) ((ts.last_ms_t + ts.last_ms_t_2 + ts.last_ms_t_1) / 3.0);
}

void Bone_Monitoring_Prototype(std::vector<RE::Actor*> actors) {
    lb_log_info("bone tracking started");

    auto default_min_stroke = 0.25; // TODO

    // settings for vaginal penetration
    std::string actorNode = "Penis_01";
    std::string playerNode = "Pelvis_skin";
    float max_distance_penetration = 14.5;
    float min_distance_penetration = 6.5;

    // TODO settings for oral penetration
    // TODO settings for anal penetration
    
    float lastDist = 0;
    bool direction_positive = false;
    int currentHandle = boneThreadHandle;
    int i = 0;

    int strokes = 0;
    timings ts;
    auto last_stroke = std::chrono::system_clock::now();
    
    float last_inward_dist = 1000.0; // distance for full penetration
    float last_outward_dist = 1000.0; // distance for least penetration

    RE::Actor* player = RE::PlayerCharacter::GetSingleton();
    if (player == NULL) {
        lb_log_error("no player");
        return;
    }
    
    lb_stroke(500, 0.0);
    while (boneThreadHandle == currentHandle)
    {
        bool isPenetrating = false;
        float minStroke = 0.0;
        float maxStroke = 1.0;


        for (RE::Actor* actor : actors)
        {
            if (player != actor)
            {
                // Leito Cowgirl 2
                if (last_inward_dist < max_distance_penetration || last_outward_dist < max_distance_penetration)
                {
                    isPenetrating = true;
                    maxStroke = 1 - ((max_distance_penetration - last_outward_dist) / (max_distance_penetration - min_distance_penetration));
                    if (maxStroke > 1.0)
                    {
                        maxStroke = 1.0;
                    }        
                    minStroke = (last_inward_dist - min_distance_penetration) / (max_distance_penetration - min_distance_penetration);
                    if (minStroke < 0.0)
                    {
                        minStroke = 0.0;
                    }

                    float min_str_dist = 0.25;
                    // lb_log_error( std::format("min:{}  max:{} diff: {}, abs {} ", minStroke, maxStroke, minStroke - maxStroke, fabs(minStroke - maxStroke) ));
                    if (fabs(maxStroke - minStroke) < min_str_dist)
                    {
                        // lb_log_error( std::format("min stroke distance too low min:{}  max:{}", minStroke, maxStroke ));
                        if (maxStroke < min_str_dist)
                        {
                            maxStroke = min_str_dist;
                            minStroke = 0.0;    
                        }
                        if (minStroke > (1.0 - min_str_dist))
                        {
                            maxStroke = 1.0;
                            minStroke = 1.0 - min_str_dist; 
                        }
                        if (maxStroke < 0.5)
                        {
                            maxStroke = minStroke + min_str_dist;
                        }
                        if (minStroke > 0.5)
                        {
                            minStroke = maxStroke - min_str_dist;
                        }
                        // lb_log_error( std::format("adapted: {}  max:{}", minStroke, maxStroke ));
                    }
                }
                

                RE::NiAVObject* actorPenis = actor->Get3D()->GetObjectByName(actorNode);
                if (actorPenis == NULL)
                {
                    lb_log_error("penis is null");
                    continue;
                }
                RE::NiAVObject* playerPelvis = player->Get3D()->GetObjectByName(playerNode);
                if (playerPelvis == NULL)
                {
                    lb_log_error("pelvis is null");
                    continue;
                }
                auto dist = distance( actorPenis, playerPelvis, player, actor );
                auto diff = dist - lastDist;

                bool doLog = false;
                if (diff > 0 && !direction_positive)
                {
                    direction_positive = true;
                    auto avg_ms = CalculateDurationMS( ts, last_stroke );
                    last_outward_dist = dist;

                    lb_log_info( std::format("NOW INWARD MOVE -----> {} ms", avg_ms) );
                    if (strokes >= 5 && isPenetrating)
                    {
                        lb_stroke(avg_ms - 25, 1.0 - minStroke);
                    }
                    strokes++;

                    doLog = true;
                }
                else if (diff < 0 && direction_positive)
                {
                    direction_positive = false;    
                    auto avg_ms = CalculateDurationMS( ts, last_stroke );
                    last_inward_dist = dist;

                    lb_log_info( std::format("NOW OUTWARD MOVE <----- {} ms", avg_ms) );
                    if (strokes >= 5 && isPenetrating)
                    {
                        lb_stroke(avg_ms - 25, 1.0 - maxStroke);
                    }
                    strokes++; 

                    doLog = true;
                }
                if (lastDist * dist < 0)
                {
                    lb_log_info("PENETRATION");
                    doLog = true;
                }
                if (doLog) {
                    
                    lb_log_info( std::format( "dist {}",dist) );
                    // logPosition( actorPenis, playerPelvis, player, actor );
                }
                  
            
                // distances[ i++ % 1024 ] = dist - lastDist;
                // lb_log_info( std::format("distance: {}", dist) );
                // lb_log_info( std::format("diff: {}", dist - lastDist) );
                lastDist = dist;

                // if (i % 1024 == 0)
                // {
                //     lb_log_info( std::format("DUMPING DIFFS:") );
                //     for (size_t i = 0; i < 1024; i++)
                //     {
                //         lb_log_info( std::format("{}", distances[ i ] ) );
                //     }
                // }
            }
        }
        
        std::this_thread::sleep_for(50ms);
    } 
}